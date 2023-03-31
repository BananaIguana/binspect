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
    pub fn header_rect(full_frame: Rect) -> Rect
    {
        Rect {
            x: 0,
            y: 0,
            width: full_frame.width,
            height: full_frame.height - 2,
        }
    }

    pub fn draw_header(&self) -> Paragraph
    {
        let spans = Spans::from(vec![
            Span::styled("Filename: ", Style::default().fg(Color::DarkGray)),
            Span::styled(self.input.filename(), Style::default()),
            Span::raw("  |  "),
            Span::styled("Mode: ", Style::default().fg(Color::DarkGray)),
            Span::styled("Test", Style::default()),
            Span::raw("  |  "),
            Span::styled("Frame: ", Style::default().fg(Color::DarkGray)),
            Span::styled(self.redraw_count.to_string(), Style::default()),
        ]);

        let paragraph = Paragraph::new(spans)
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        paragraph
    }
}
