use crate::{input::Input, view::View};

mod input;
mod utils;
mod view;

#[quit::main]
fn main() -> Result<(), std::io::Error>
{
    let input = Input::new();

    let mut view = View::from_input(input);

    while !view.poll_event()
    {
        view.redraw()?;
    }

    Ok(())
}
