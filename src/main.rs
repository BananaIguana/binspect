use crate::args::Args;

mod args;

#[quit::main]
fn main()
{
    Args::new();

    println!("Hello, world!");
}
