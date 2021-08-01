use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use stpl::Render;

pub fn render_content<C: Render + 'static>(content: C) -> Result<String> {
    let mut html_buffer = vec![];
    let mut renderer = stpl::html::Renderer::new(&mut html_buffer);

    let with_base = crate::html::base::base(&crate::html::base::Data::new("Title:)"), content);
    with_base.render(&mut renderer).wrap_err("Render issue")?;

    String::from_utf8(html_buffer).wrap_err("Utf-8 conversion issue")
}
