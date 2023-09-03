use {
    crate::view::View,
    std::cmp::min,
    tui::{
        style::{Color, Style},
        text::Spans,
        widgets::{Block, Borders, Paragraph, Wrap},
    },
};

impl View
{
    pub fn draw_data_view(&self, area: usize) -> Paragraph
    {
        let hex_data = self.input.hex_data();

        let slice = &hex_data[0..min(hex_data.len(), area)];

        let spans = Spans::from(slice);

        let paragraph = Paragraph::new(spans)
            .style(Style::default().fg(Color::Gray).bg(Color::Black))
            .block(
                Block::default()
                    .borders(Borders::all())
                    .border_style(Style::default().fg(Color::DarkGray).bg(Color::Black))
                    .style(Style::default().bg(Color::Black)),
            )
            .wrap(Wrap { trim: false });

        paragraph
    }
}
