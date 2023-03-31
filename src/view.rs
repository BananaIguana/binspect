use {
    crate::{
        input::Input,
        utils::{Edges, RectUtils},
    },
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    std::{cell::RefCell, io::Stdout, rc::Rc},
    tui::{backend::CrosstermBackend, Terminal},
};

mod data_view;
mod footer;
mod header;

/// Console view rendering structure.
pub struct View
{
    terminal: Rc<RefCell<Terminal<CrosstermBackend<Stdout>>>>,
    redraw_count: u32,
    input: Input,
}

impl View
{
    /// Create a `View` object.
    pub fn from_input(input: Input) -> Self
    {
        let terminal = Self::create().expect("Failed to create terminal backend.");
        let terminal = Rc::new(RefCell::new(terminal));

        let mut ret = Self {
            terminal,
            input,
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

        let header = self.draw_header();
        let data_view = self.draw_data_view();
        let footer = self.draw_footer();

        let mut terminal = self.terminal.borrow_mut();

        terminal.draw(|f| {
            let border = f.size().inset(1, Edges::TOP | Edges::BOTTOM);
            let top = View::header_rect(f.size());
            let bottom = View::footer_rect(f.size());

            f.render_widget(header, top);
            f.render_widget(data_view, border);
            f.render_widget(footer, bottom);
        })?;

        Ok(())
    }

    /// Destroy the terminal backend.
    fn destroy(&mut self) -> Result<(), std::io::Error>
    {
        // restore terminal
        disable_raw_mode()?;

        execute!(
            self.terminal.borrow_mut().backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        self.terminal.borrow_mut().show_cursor()?;

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
