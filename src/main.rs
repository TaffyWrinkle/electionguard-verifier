use serde_json::from_reader;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::{stdin, BufReader};
use structopt::StructOpt;

mod election;

#[structopt(
    name = "electionguard-verify",
    about = "Verify the results of an election."
)]
#[derive(StructOpt)]
struct Options {
    /// The path to the JSON file containing the election results.
    /// We read from STDIN if not present.
    #[structopt(parse(from_os_str))]
    #[structopt(short = "i", long = "input")]
    input: Option<std::path::PathBuf>,
}

#[derive(Debug)]
enum Error {
    IO(io::Error),
    JSON(serde_json::Error),
    Validation(Vec<election::Error>),
}

impl From<Vec<election::Error>> for Error {
    fn from(error: Vec<election::Error>) -> Error {
        Error::Validation(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IO(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::JSON(error)
    }
}

fn main() -> Result<(), Error> {
    let opt = Options::from_args();

    let reader: Box<Read> = match opt.input {
        None => Box::new(stdin()),
        Some(path) => Box::new(File::open(path)?),
    };

    let input: election::Results = from_reader(BufReader::new(reader))?;

    let errors = input.validate();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into())
    }
}
