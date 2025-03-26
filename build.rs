// SPDX-FileCopyrightText: 2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Build faircamp with (e.g.) FAIRCAMP_PKG_VERSION=2.0.0~pre1 to override
/// the version that is displayed and reported in resulting builds.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let version_detailed;
    let version_display;
    if let Ok(override_version) = env::var("FAIRCAMP_PKG_VERSION") {
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

    println!("cargo:rerun-if-env-changed=FAIRCAMP_PKG_VERSION");
    println!("cargo:rustc-env=FAIRCAMP_REVISION={revision}");
    println!("cargo:rustc-env=FAIRCAMP_VERSION_DETAILED={version_detailed}");
    println!("cargo:rustc-env=FAIRCAMP_VERSION_DISPLAY={version_display}");

    preprocess_js("browser.js", include_str!("src/assets/browser.js"), "FAIRCAMP_BROWSER_JS");
    preprocess_js("clipboard.js", include_str!("src/assets/clipboard.js"), "FAIRCAMP_CLIPBOARD_JS");
    preprocess_js("embeds.js", include_str!("src/assets/embeds.js"), "FAIRCAMP_EMBEDS_JS");
    preprocess_js("player.js", include_str!("src/assets/player.js"), "FAIRCAMP_PLAYER_JS");
}

fn preprocess_js(filename: &str, input: &str, varname: &str) {
    let target_path = Path::new(&env::var("OUT_DIR").unwrap())
        .join(filename)
        .to_str()
        .unwrap()
        .to_string();

    #[cfg(feature = "no-minify")]
    let _ = fs::write(&target_path, input);

    #[cfg(not(feature = "no-minify"))]
    let _ = fs::write(&target_path , minify::minify(input));

    println!("cargo:rustc-env={varname}={target_path}");
}

#[cfg(not(feature = "no-minify"))]
mod minify {
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
