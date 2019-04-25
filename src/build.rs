use pulldown_cmark::{Parser, Options, html};
use std::fs::{ File, read_dir };
use std::env;
use std::path::{ Path, PathBuf };

use exitfailure::ExitFailure;
use failure::ResultExt;

// fn split_page_content(file_path: &Path, content: &str) -> Result<(PageFrontMatter, String)> {
//     let (front_matter, content) = split_content(file_path, content)?;
//     let meta = PageFrontMatter::parse(&front_matter).map_err(|e| {
//         Error::chain(
//             format!("Error when parsing front matter of page `{}`", file_path.to_string_lossy()),
//             e,
//         )
//     })?;
//     Ok((meta, content))
// }

fn build_markdown<P: AsRef<Path>>(path: P) -> Result<(), ExitFailure> {
    let path = path.as_ref();

    for entry in read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        println!("{}", entry.path().display());
    }

    let markdown_input: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";
    println!("Parsing the following markdown string:\n{}", markdown_input);

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    let expected_html: &str = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    assert_eq!(expected_html, &html_output);

    // Write result to stdout.
    println!("\nHTML output:\n{}", &html_output);

    Ok(())
}

pub fn build_site (source_dir: Option<&str>, destination_dir: &str) -> Result<(), ExitFailure> {
    let bp = source_dir.map(PathBuf::from).unwrap_or(env::current_dir().unwrap());

    build_markdown(bp)?;

    Ok(())
}
