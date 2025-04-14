// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Build faircamp with (e.g.) FAIRCAMP_VERSION=2.0.0~pre1 to override
/// the version that is displayed and reported in resulting builds.

use std::env;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::process::Command;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

fn precompute_hash(content: &[u8], hash_varname: &str) {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let hash = hasher.finish();
    let hash_encoded = URL_SAFE_NO_PAD.encode(hash.to_le_bytes());
    println!("cargo:rustc-env={hash_varname}={hash_encoded}");
}

fn main() {
    let version_detailed;
    let version_display;
    if let Ok(override_version) = env::var("FAIRCAMP_VERSION") {
        version_detailed = override_version.clone();
        version_display = override_version;
    } else {
        version_detailed = env!("CARGO_PKG_VERSION").to_string();
        version_display = concat!(env!("CARGO_PKG_VERSION_MAJOR"), '.', env!("CARGO_PKG_VERSION_MINOR")).to_string();
    }

    let mut git = Command::new("git");
    git.args(["rev-parse", "--short", "HEAD"]);
    let revision = match git.output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(_) => String::from("unknown revision")
    };

    println!("cargo:rerun-if-env-changed=FAIRCAMP_VERSION");
    println!("cargo:rustc-env=FAIRCAMP_REVISION={revision}");
    println!("cargo:rustc-env=FAIRCAMP_VERSION_DETAILED={version_detailed}");
    println!("cargo:rustc-env=FAIRCAMP_VERSION_DISPLAY={version_display}");

    #[cfg(all(feature = "libvips", feature = "minify"))]
    println!("cargo:rustc-env=FAIRCAMP_FEATURES=compiled with libvips and minified assets");
    #[cfg(all(feature = "libvips", not(feature = "minify")))]
    println!("cargo:rustc-env=FAIRCAMP_FEATURES=compiled with libvips, without minified assets");
    #[cfg(all(not(feature = "libvips"), feature = "minify"))]
    println!("cargo:rustc-env=FAIRCAMP_FEATURES=compiled without libvips, with minified assets");
    #[cfg(all(not(feature = "libvips"), not(feature = "minify")))]
    println!("cargo:rustc-env=FAIRCAMP_FEATURES=compiled without libvips or minified assets");

    preprocess_assets();
}

fn preprocess_assets() {
    precompute_hash(
        include_bytes!("src/assets/favicon_dark.png"),
        "FAIRCAMP_FAVICON_DARK_PNG_HASH"
    );

    precompute_hash(
        include_bytes!("src/assets/favicon_light.png"),
        "FAIRCAMP_FAVICON_LIGHT_PNG_HASH"
    );

    precompute_hash(
        include_bytes!("src/assets/favicon.svg"),
        "FAIRCAMP_FAVICON_SVG_HASH"
    );

    preprocess_css(
        "embeds.css",
        Some("FAIRCAMP_EMBEDS_CSS_HASH"),
        include_str!("src/assets/embeds.css"),
        "FAIRCAMP_EMBEDS_CSS"
    );

    preprocess_css(
        "missing_image_descriptions.css",
        None,
        include_str!("src/assets/missing_image_descriptions.css"),
        "FAIRCAMP_MISSING_IMAGE_DESCRIPTIONS_CSS"
    );

    preprocess_css(
        "site.css",
        None,
        include_str!("src/assets/site.css"),
        "FAIRCAMP_SITE_CSS"
    );

    preprocess_css(
        "theming_widget.css",
        None,
        include_str!("src/assets/theming_widget.css"),
        "FAIRCAMP_THEMING_WIDGET_CSS"
    );

    preprocess_js(
        "browser.js",
        None,
        include_str!("src/assets/browser.js"),
        "FAIRCAMP_BROWSER_JS"
    );

    preprocess_js(
        "clipboard.js",
        Some("FAIRCAMP_CLIPBOARD_JS_HASH"),
        include_str!("src/assets/clipboard.js"),
        "FAIRCAMP_CLIPBOARD_JS"
    );

    preprocess_js(
        "embeds.js",
        None,
        include_str!("src/assets/embeds.js"),
        "FAIRCAMP_EMBEDS_JS"
    );

    preprocess_js(
        "player.js",
        None,
        include_str!("src/assets/player.js"),
        "FAIRCAMP_PLAYER_JS"
    );
}

fn preprocess_css(
    filename: &str,
    hash_varname: Option<&str>,
    input: &str,
    varname: &str
) {
    let target_path = Path::new(&env::var("OUT_DIR").unwrap())
        .join(filename)
        .to_str()
        .unwrap()
        .to_string();

    #[cfg(not(feature = "minify"))]
    let output = input;

    #[cfg(feature = "minify")]
    let output = minify_css::minify(input);

    let _ = fs::write(&target_path, &output);

    println!("cargo:rustc-env={varname}={target_path}");

    if let Some(hash_varname) = hash_varname {
        precompute_hash(output.as_bytes(), hash_varname);
    }
}

fn preprocess_js(
    filename: &str,
    hash_varname: Option<&str>,
    input: &str,
    varname: &str
) {
    let target_path = Path::new(&env::var("OUT_DIR").unwrap())
        .join(filename)
        .to_str()
        .unwrap()
        .to_string();

    #[cfg(not(feature = "minify"))]
    let output = input;

    #[cfg(feature = "minify")]
    let output = minify_js::minify(input);

    let _ = fs::write(&target_path, &output);

    println!("cargo:rustc-env={varname}={target_path}");

    if let Some(hash_varname) = hash_varname {
        precompute_hash(output.as_bytes(), hash_varname);
    }
}

#[cfg(feature = "minify")]
mod minify_css {
    use lightningcss::stylesheet::{StyleSheet, ParserOptions, MinifyOptions, PrinterOptions};

    pub fn minify(input: &str) -> String {
        let mut stylesheet = StyleSheet::parse(input, ParserOptions::default()).unwrap();

        stylesheet.minify(MinifyOptions::default()).unwrap();

        let printer_options = PrinterOptions {
            minify: true,
            ..PrinterOptions::default()
        };

        stylesheet
            .to_css(printer_options)
            .unwrap()
            .code
    }
}

#[cfg(feature = "minify")]
mod minify_js {
    const SOURCE_TYPE: SourceType = SourceType::cjs();

    use oxc_allocator::Allocator;
    use oxc_codegen::{CodeGenerator, CodegenOptions};
    use oxc_mangler::MangleOptions;
    use oxc_minifier::{CompressOptions, Minifier, MinifierOptions};
    use oxc_parser::Parser;
    use oxc_span::SourceType;
    use oxc_transformer::ESTarget;

    pub fn minify(input: &str) -> String {
        let mut allocator = Allocator::default();

        let first_pass_result = minify_pass(&allocator, input);

        allocator.reset();

        minify_pass(&allocator, &first_pass_result)
    }

    fn minify_pass(allocator: &Allocator, input: &str) -> String {
        let parser_return = Parser::new(allocator, input, SOURCE_TYPE)
            .parse();

        let mut program = parser_return.program;

        let compress_options = CompressOptions {
            target: ESTarget::ES2020,
            ..CompressOptions::default()
        };

        let minifier_options = MinifierOptions {
            mangle: Some(MangleOptions::default()),
            compress: Some(compress_options),
        };

        let minifier_return = Minifier::new(minifier_options)
            .build(allocator, &mut program);

        let codegen_options = CodegenOptions {
            minify: true,
            ..CodegenOptions::default()
        };

        CodeGenerator::new()
            .with_options(codegen_options)
            .with_scoping(minifier_return.scoping)
            .build(&program)
            .code
    }
}
