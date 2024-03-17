use std::{fmt::{self, Display, Formatter}, fs::{self, File as fsFile}};

pub struct File {
    data: fsFile,
    path: String,
    buffer: String,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n> {}\n", self.path, self.buffer.replace('\n', "\n> "))
    }
}

impl File {
    pub fn create(path: &str) -> Result<Self, String> {
        Ok(File {
            data: fsFile::open(path).map_err(|err| err.to_string())?,
            path: path.to_string(),
            buffer: fs::read_to_string(path).map_err(|err| err.to_string())?.trim_end().to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_PATH: &str = "examples/minimum_required.fus";

    #[test]
    fn test_file_create() {
        assert!(File::create(TEST_PATH).is_ok());
        assert!(File::create("wrong_path").is_err());
    }

    #[test]
    fn test_file_display() {
        let output: &str = &format!("{}", File::create(TEST_PATH).unwrap());
        assert_eq!(output, "examples/minimum_required.fus:\n> fn main {}\n")
    }
}
