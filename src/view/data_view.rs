use {
    crate::view::View,
    tui::{
        style::Style,
        widgets::{Block, Borders},
    },
};

impl View
{
    pub fn draw_data_view(&self) -> Block
    {
        Block::default()
            .borders(Borders::all())
            .border_style(Style::default())
    }
}
