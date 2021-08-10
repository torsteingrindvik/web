use stpl::Render;

/// Wrap the given content in a div which uses the class given.
pub fn div_wrap<C: Render + 'static>(class: String, content: C) -> impl Render {
    use stpl::html::*;

    div.class(class)(content)
}
