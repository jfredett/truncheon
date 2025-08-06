use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::Wrap;

use ratatui::buffer::Buffer;


#[derive(Debug, Default, Clone)]
pub struct CanvasPlaceholder {
}

// Features:
// 1. Renders to a specific size
// 2. mostly doesn't not work
// Planned Features:
// 1. tickers the text if size is too small. Maybe even DVD-logos in boxes?



/// A Placeholder widget that renders to the specific size given, with a border.
/// It contains the text "Placeholder" in the center of the widget.

impl StatefulWidget for &CanvasPlaceholder {
    type State = ();

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        Widget::render(self, area, buf);
    }
}

impl Widget for &CanvasPlaceholder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let widget = Canvas::default()
            .x_bounds([-100.0, 100.0])
            .y_bounds([-100.0, 100.0])
            .paint(|ctx| {
                ctx.draw(&Rectangle {
                    x: 10.0,
                    y: 20.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Red,
                });
            });

        Widget::render(&widget, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    mod rendering {
        use super::*;

    }
}

