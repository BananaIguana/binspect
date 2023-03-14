use {
    std::path::Path,
    user_error::{UserFacingError, UFE},
};

#[allow(unused_variables)]
#[allow(dead_code)]
pub struct Args
{
    file: String,
    data: Vec<u8>,
}

impl Args
{
    pub fn new() -> Self
    {
        let args: Vec<String> = std::env::args().collect();

        if args.len() == 1
        {
            UserFacingError::new("Arguments")
                .reason("No input argument supplied.")
                .help("Try: binspect [file]")
                .print();

            quit::with_code(1);
        }

        if args.len() > 2
        {
            UserFacingError::new("Arguments")
                .reason("Too many supplied.")
                .help("Try: binspect [file]")
                .print();

            quit::with_code(1);
        }

        let file = args.last().unwrap().to_string();

        if let Ok(data) = Self::load(&file)
        {
            Args { file, data }
        }
        else
        {
            UserFacingError::new("Arguments")
                .reason("Cannot load input file.")
                .help("Hint: Check the path is valid.")
                .print();

            quit::with_code(1);
        }
    }

    fn load(input: &str) -> Result<Vec<u8>, std::io::Error>
    {
        let path = Path::new(input);
        let _file = std::fs::File::open(path)?;

        Ok(vec![])
    }
}
