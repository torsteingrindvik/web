use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use stpl::Render;

/// Wrap content in a div with some class.
// Note: Yes, it's overkill to use a template for this.
// But we want to learn the template workflow, so we do it anyway.
pub fn div_wrap<C: Render + 'static>(class: &str, content: C) -> Result<String> {
    let mut html_buffer = vec![];
    let mut renderer = stpl::html::Renderer::new(&mut html_buffer);

    let wrapped = crate::html::util::div_wrap(class.to_string(), content);
    wrapped.render(&mut renderer).wrap_err("Render issue")?;

    String::from_utf8(html_buffer).wrap_err("Utf-8 conversion issue")
}

/// Render HTML using the default base.
pub fn render_content<C: Render + 'static>(page_title: &str, content: C) -> Result<String> {
    let mut html_buffer = vec![];
    let mut renderer = stpl::html::Renderer::new(&mut html_buffer);

    let with_base = crate::html::base::base(page_title, content);
    with_base.render(&mut renderer).wrap_err("Render issue")?;

    String::from_utf8(html_buffer).wrap_err("Utf-8 conversion issue")
}
