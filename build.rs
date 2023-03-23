use indoc::formatdoc;
use pulldown_cmark::{html, Parser};
use slug::slugify;
use std::env;
use std::fs::{self, DirEntry};
use std::path::Path;

struct Docs {
	index: Page,
	reference: Vec<Page>,
	topics: Vec<Page>
}

#[derive(PartialEq)]
struct Page {
	content: String,
	slug: String,
	title: String
}

const MANUAL_DIR: &str = "src/manual";

pub fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown);
    
    html::push_html(&mut html_output, parser);
    
    html_output
}

pub fn main() {
	println!("cargo:rerun-if-changed={MANUAL_DIR}");

	let docs = read_docs();

	let out_dir = env::var_os("OUT_DIR").unwrap();
    let manual_out_dir = Path::new(&out_dir).join("manual");

	if manual_out_dir.exists() {
		fs::remove_dir_all(&manual_out_dir).unwrap();
	}

	fs::create_dir(&manual_out_dir).unwrap();

	render_page(
		&manual_out_dir,
		&docs,
		&docs.index,
		docs.topics.first()
	);

	let mut topics_iter = docs.topics.iter().peekable();
	while let Some(page) = topics_iter.next() {
		render_page(
			&manual_out_dir,
			&docs,
			page,
			topics_iter.peek().copied().or_else(|| docs.reference.first())
		);
	}

	let mut reference_iter = docs.reference.iter().peekable();
	while let Some(page) = reference_iter.next() {
		render_page(
			&manual_out_dir,
			&docs,
			page,
			reference_iter.peek().copied()
		);
	}

    fs::write(
        manual_out_dir.join("favicon.svg"),
        include_bytes!("src/assets/favicon.svg")
    ).unwrap();

    fs::write(
        manual_out_dir.join("favicon_dark.png"),
        include_bytes!("src/assets/favicon_dark.png")
    ).unwrap();

    fs::write(
        manual_out_dir.join("favicon_light.png"),
        include_bytes!("src/assets/favicon_light.png")
    ).unwrap();

    let text_color = String::from("hsl(0, 0%, 100%)");
    let logo_svg = format!(
        include_str!("src/icons/logo.svg"),
        text_color = text_color
    );
    fs::write(manual_out_dir.join("logo.svg"), logo_svg).unwrap();

	fs::copy(
		Path::new(MANUAL_DIR).join("fira-mono-v14-latin_latin-ext-regular.woff2"),
		manual_out_dir.join("fira-mono-v14-latin_latin-ext-regular.woff2")
	).unwrap();

	fs::copy(
		Path::new(MANUAL_DIR).join("titillium-web-v15-latin_latin-ext-regular.woff2"),
		manual_out_dir.join("titillium-web-v15-latin_latin-ext-regular.woff2")
	).unwrap();

	fs::copy(
		Path::new(MANUAL_DIR).join("titillium-web-v15-latin_latin-ext-italic.woff2"),
		manual_out_dir.join("titillium-web-v15-latin_latin-ext-italic.woff2")
	).unwrap();

	fs::copy(
		Path::new(MANUAL_DIR).join("styles.css"),
		manual_out_dir.join("styles.css")
	).unwrap();
}

fn layout(body: &str, title: &str, docs: &Docs, active_page: &Page) -> String {
	let reference = docs.reference
		.iter()
		.map(|page| {
			let active = if page == active_page { r#"class="active" "# } else { "" };
			let slug = &page.slug;
			let title = &page.title;

			format!(r#"<a {active}href="{slug}.html">{title}</a>"#)
		})
		.collect::<Vec<String>>()
		.join("\n");

	// TODO: DRY, exactly the same as for reference above
	let topics = docs.topics
		.iter()
		.map(|page| {
			let active = if page == active_page { r#"class="active" "# } else { "" };
			let slug = &page.slug;
			let title = &page.title;

			format!(r#"<a {active}href="{slug}.html">{title}</a>"#)
		})
		.collect::<Vec<String>>()
		.join("\n");

	let index_active = if active_page == &docs.index { r#" class="active""# } else { "" };

	formatdoc!(r##"
		<!doctype html>
		<html>
			<head>
				<title>{title}</title>
				<meta charset="utf-8">
		        <meta name="description" content="{title}">
		        <meta name="viewport" content="width=device-width, initial-scale=1">
		        <link href="favicon.svg" rel="icon" type="image/svg+xml">
		        <link href="favicon_light.png" rel="icon" type="image/png" media="(prefers-color-scheme: light)">
		        <link href="favicon_dark.png" rel="icon" type="image/png"  media="(prefers-color-scheme: dark)">
		        <link href="styles.css?0" rel="stylesheet">
			</head>
			<body>
				<header>
					<a class="title" href="index.html">
						<span{index_active}>Faircamp Manual</span>
						<img src="logo.svg">
					</a>
					<a class="open_nav" href="#nav">☰</a>
				</header>

				<div class="split">
					<nav id="nav">
						<div class="nav_inner">
							<a class="close_nav" href="#">✕</a>

							<span>Topics</span>
							{topics}

							<span>Reference</span>
							{reference}
						</div>
					</nav>
					<main>
						{body}
					</main>
				</div>
			</body>
		</html>
	"##)
}

fn read_docs() -> Docs {
	let index_path = Path::new(MANUAL_DIR).join("index.md");
	let index_markdown = fs::read_to_string(index_path).unwrap();
	let index_content = markdown_to_html(&index_markdown);
	
	let index = Page {
		content: index_content,
		slug: String::from("index"),
		title: String::from("Faircamp Manual")
	};

	let reference = read_pages(&Path::new(MANUAL_DIR).join("reference"));
	let topics = read_pages(&Path::new(MANUAL_DIR).join("topics"));

	Docs {
		index,
		reference,
		topics
	}
}

fn read_pages(dir: &Path) -> Vec<Page> {
	let mut pages: Vec<DirEntry> = fs::read_dir(dir)
		.unwrap()
		.flatten()
		.collect();

	pages.sort_by_key(|dir_entry| dir_entry.file_name());

	pages
		.into_iter()
		.map(|dir_entry| {
			let path = dir_entry.path();
			let file_stem = path.file_stem().unwrap().to_string_lossy();

			let title = match file_stem.split_once(' ') {
				Some((prefix, suffix)) => {
					match prefix.parse::<usize>() {
						Ok(_) => suffix.to_string(),
						Err(_) => file_stem.to_string()
					}
				}
				None => file_stem.to_string()
			};

			let content_markdown = fs::read_to_string(&path).unwrap();
			let content = markdown_to_html(&content_markdown);

			let slug = slugify(&title);

			Page { content, slug, title }
		})
		.collect()
}

fn render_page(
	manual_out_dir: &Path,
	docs: &Docs,
	page: &Page,
	next_page: Option<&Page>
) {
	let content = &page.content;

	let body = if let Some(next_page) = next_page {
		let next_page_slug = &next_page.slug;
		let next_page_title = &next_page.title;

		formatdoc!(r#"
			{content}
			
			<div class="next_page">
				Next page: <a href="{next_page_slug}.html">{next_page_title}</a>
			</div>
		"#)
	} else {
		content.clone()
	};

	let html = layout(&body, &page.title, docs, page);

	let out_path = manual_out_dir.join(&page.slug).with_extension("html");

	fs::write(out_path, html).unwrap();
}
