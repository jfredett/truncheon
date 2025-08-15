use std::path::Path;

use tera::{Context, Tera};

/// A container for a template which will be rendered into an SVG.
///
/// The render 'pipeline' is straightforward:
///
/// 1. Create the SVGTemplate, by default, it renders a blank SVG.
/// 2. Add 'Element's to the template, these are classes which implement a 'draw' method which
///    takes an SVGTemplate as input and updates it to properly render whatever element. These can
///    use the `tera` templating system and it's [[tera::Context]] to dynamically create stuff in
///    the SVG
/// 3. Render the template, which returns a string containing the actual SVG to be rendered by
///    [[SVG]] or other thing.
///
///
#[derive(Debug, Clone)]
pub struct SVGTemplate {
    // the template-context should be updated here, the render is called afterward, that way it can
    // potentially update multiple times between refreshes / avoids rendering until it's needed.
    context: Context,
    content: String
}

impl SVGTemplate {
    pub fn from_str(s: &str) -> Self {
        Self::new(s.to_string())
    }

    pub fn from_file(p: &Path) -> Self {
        let s = std::fs::read_to_string(p).unwrap();
        Self::from_str(&s)
    }

    pub fn new(s: String) -> Self {
        Self {
            context: Context::default(),
            content: s
        }
    }

    pub fn render(&self) -> String {
        // TODO: Accept a context, render a template
        let result = Tera::one_off(&self.content, &self.context, false);
        result.unwrap()
    }
}

