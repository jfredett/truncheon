use std::path::Path;

use tera::{Context, Tera};

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

