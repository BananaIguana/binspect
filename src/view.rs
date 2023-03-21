use {
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    std::io::Stdout,
    tui::{
        backend::CrosstermBackend,
        layout::Alignment,
        style::{Color, Style},
        text::Spans,
        widgets::{Block, Borders, Paragraph, Wrap},
        Terminal,
    },
};

/// Console view rendering structure.
pub struct View
{
    terminal: Terminal<CrosstermBackend<Stdout>>,
    redraw_count: u32,
}

impl View
{
    /// Create a `View` object.
    pub fn new() -> Self
    {
        let terminal = Self::create().expect("Failed to create terminal backend.");

        let mut ret = Self {
            terminal,
            redraw_count: 0,
        };

        ret.redraw().expect("Failed to perform initial draw call.");
        ret
    }

    /// Creates the terminal backend
    fn create() -> Result<Terminal<CrosstermBackend<Stdout>>, std::io::Error>
    {
        enable_raw_mode()?;

        let mut stdout = std::io::stdout();

        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(terminal)
    }

    /// Poll & process user events.
    pub fn poll_event(&self) -> bool
    {
        if let Event::Key(key) = event::read().unwrap()
        {
            matches!(key.code, KeyCode::Char('q'))
        }
        else
        {
            false
        }
    }

    /// Redraw the view.
    pub fn redraw(&mut self) -> Result<(), std::io::Error>
    {
        self.redraw_count += 1;

        let spans = vec![
            Spans::from(format!("Redraw count = {}", self.redraw_count)),
            Spans::from("Press 'Q' to quit."),
        ];

        let paragraph = Paragraph::new(spans)
            .block(
                Block::default()
                    .title("Placeholder Frame")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        self.terminal.draw(|f| {
            let size = f.size();
            f.render_widget(paragraph, size);
        })?;

        Ok(())
    }

    /// Destroy the terminal backend.
    fn destroy(&mut self) -> Result<(), std::io::Error>
    {
        // restore terminal
        disable_raw_mode()?;

        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        self.terminal.show_cursor()?;

        Ok(())
    }
}

impl Drop for View
{
    fn drop(&mut self)
    {
        self.destroy()
            .expect("Failed to clean-up terminal backend.");
    }
}
