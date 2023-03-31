use {
    crate::view::View,
    tui::{
        layout::{Alignment, Rect},
        style::{Color, Style},
        text::{Span, Spans},
        widgets::{Paragraph, Wrap},
    },
};

impl View
{
    pub fn footer_rect(full_frame: Rect) -> Rect
    {
        Rect {
            x: 0,
            y: full_frame.height - 1,
            width: full_frame.width,
            height: 1,
        }
    }

    pub fn draw_footer(&self) -> Paragraph
    {
        let spans = Spans::from(vec![Span::styled(
            "'Q' = Quit'",
            Style::default().fg(Color::DarkGray),
        )]);

        let paragraph = Paragraph::new(spans)
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        paragraph
    }
}
