use std::path::Path;

use serde::Serialize;
use tera::{Context, Tera};

/// A container for a template which will be rendered into an SVG.
///
/// The render 'pipeline' is straightforward:
///
/// 1. Create the SVGTemplate, by default, it renders a blank SVG.
/// 2. Add 'SVGElement's to the template, these are classes which implement a 'draw' method which
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

impl Default for SVGTemplate {
    fn default() -> Self {
        let path_to_default_svg = std::fs::canonicalize("./src/ui/widgets/svg/default.svg").ok().unwrap();
        Self::from_file(&path_to_default_svg)
    }
}

impl SVGTemplate {
    pub fn from_str(s: &str) -> Self {
        Self::new(s.to_string())
    }

    pub fn from_file(p: &Path) -> Self {
        let s = std::fs::read_to_string(p).unwrap();
        Self::from_str(&s)
    }

    pub fn new(content: String) -> Self {
        Self { context: Context::default(), content }
    }

    pub fn render(&self) -> String {
        let result = Tera::one_off(&self.content, &self.context, false);
        result.unwrap()

        // The pipeline:
        //
        // Every time an `add` occurs, the `draw` call is run immediately with the SVGTemplate as a
        // RO context. The `draw` command can query into the context for variables as needed to
        // render itself, it may itself be a tera template. It may generate more templating code.
        //
        // After all the `draw` calls happen, a final `render` run does teh `tera::one_off` and
        // renders the final SVG, which can then be handed off. This adds whatever boilerplate is
        // needed.
    }

    pub fn set_width(&mut self, width: u16) {
        self.context.insert("width", &width);
    }

    pub fn set_height(&mut self, height: u16) {
        self.context.insert("height", &height);
    }


    pub fn add(&mut self, _element: &impl SVGElement) {
        todo!();
    }
}

pub trait SVGElement where Self: Serialize  {
    fn draw(&self, template: &SVGTemplate) -> String;
}


#[derive(Serialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Rect {

}

impl SVGElement for Rect {
    fn draw(&self, _template: &SVGTemplate) -> String {
        r##"<rect x="100" y="100" width="100" height="100" rx="25" />"##.to_string()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[tracing_test::traced_test]
    fn renders_default_correctly() {
        let e = SVGTemplate::default();
        let res = e.render();

        assert_snapshot!(res);
    }
}




