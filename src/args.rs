use user_error::{UserFacingError, UFE};

pub struct Args {}

impl Args
{
    pub fn new() -> Self
    {
        let args: Vec<String> = std::env::args().collect();

        if args.len() == 1
        {
            UserFacingError::new("Runtime Error")
                .reason("No input argument supplied.")
                .help("Try: binspect [file]")
                .print();

            quit::with_code(1);
        }

        Args {}
    }
}
