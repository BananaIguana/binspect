use crate::{input::Input, view::View};

mod input;
mod view;

#[quit::main]
fn main() -> Result<(), std::io::Error>
{
    Input::new();

    let mut view = View::new();

    while !view.poll_event()
    {
        view.redraw()?;
    }

    Ok(())
}
