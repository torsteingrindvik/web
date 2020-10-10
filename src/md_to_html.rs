use anyhow::{anyhow, Result};
use lol_html::{element, html_content::ContentType, text, HtmlRewriter, Settings};
use serde::Serialize;

#[derive(Serialize)]
/// A markdown struct ready to be serialized in the format Github's REST API expects
struct Markdown<'m> {
    text: &'m str,
}

/// Embeds the given html and title into the template html and returns result
fn process_html(html: &str, title: &str) -> Result<String> {
    let mut output = vec![];

    let mut rewriter = HtmlRewriter::try_new(
        Settings {
            element_content_handlers: vec![
                element!(".markdown-body", |el| {
                    el.set_inner_content(html, ContentType::Html);
                    Ok(())
                }),
                text!("title", |t| {
                    if !t.last_in_text_node() {
                        t.replace(title, ContentType::Text);
                    }
                    Ok(())
                }),
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    )?;

    let template = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/md_template.html"));

    if let Err(e) = rewriter.write(template) {
        return Err(anyhow!("RewritingError: {:?}", e));
    };

    Ok(std::str::from_utf8(&output)?.to_string())
}

/// Given some markdown string, produce a html string
pub fn markdown_to_html(markdown: &str, title: &str) -> Result<String> {
    // We're cheating! Let Github render it for us!

    // Github wants it in Json format
    let markdown_as_json = serde_json::to_string(&Markdown { text: markdown })?;

    let client = reqwest::blocking::Client::new();
    let github_html = client
        .post("https://api.github.com/markdown")
        .body(markdown_as_json)
        .header(reqwest::header::USER_AGENT, "torsteingrindvik")
        .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
        .send()?
        .text()?;

    let html = process_html(&github_html, title)?;

    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test we're able to modify HTML to embed the Github CSS stylesheet and some small style additions
    fn test_modify_html() {
        // A simple markdown file gives the following output when given to Github's REST API:
        let html = "<h1>\n<a id=\"user-content-hello-world\" class=\"anchor\" href=\"#hello-world\" aria-hidden=\"true\"><span aria-hidden=\"true\" class=\"octicon octicon-link\"></spa
n></a>Hello World</h1>\n<p>Good to be here!</p>\n";

        let modified_html = process_html(html, "foo").unwrap();

        eprintln!("{:#?}", modified_html);
    }
}
