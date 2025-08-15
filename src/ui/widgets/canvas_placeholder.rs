use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::{canvas::{Canvas, Rectangle}, Block, StatefulWidget, Widget}};



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
    use insta::assert_debug_snapshot;
    use rstest::rstest;

    mod rendering {

        use ratatui::style::Style;

        use super::*;

        #[rstest]
        fn renders_as_expected_stateless() {
            let rect = Rect::new(0, 0, 8, 8);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = CanvasPlaceholder::default();

            Widget::render(&placeholder, rect, &mut buffer);

            assert_debug_snapshot!(buffer);
        }

        #[rstest]
        fn renders_as_expected_stateful() {
            let rect = Rect::new(0, 0, 8, 8);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = CanvasPlaceholder::default();

            StatefulWidget::render(&placeholder, rect, &mut buffer, &mut ());

            assert_debug_snapshot!(buffer);
        }
    }
}

