use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::Wrap;

use ratatui::buffer::Buffer;


#[derive(Debug, Default, Clone)]
pub struct CanvasPlaceholder {
}

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
            .block(Block::bordered().title("Canvas"))
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

