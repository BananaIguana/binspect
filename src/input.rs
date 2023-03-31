use {
    std::{io::Read, path::Path},
    user_error::{UserFacingError, UFE},
};

#[allow(unused_variables)]
#[allow(dead_code)]
pub struct Input
{
    file: String,
    data: Vec<u8>,
}

impl Input
{
    /// Create an `Input` object. This function parses the program arguments and loads the
    /// specified file.
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
            Input { file, data }
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

    /// Loads a file from disk to an array.
    fn load(input: &str) -> Result<Vec<u8>, std::io::Error>
    {
        let path = Path::new(input);
        let mut file = std::fs::File::open(path)?;
        let metadata = std::fs::metadata(path)?;

        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn filename(&self) -> &str
    {
        self.file.as_str()
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &[u8]
    {
        &self.data
    }
}
