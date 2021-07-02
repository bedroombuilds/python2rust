use std::fs::File;
use std::io::{self, Write};

struct FileManager {
    filename: std::path::PathBuf,
    mode: char,
    file: Option<File>,
}

impl FileManager {
    fn new(filename: impl AsRef<std::path::Path>, mode: char) -> FileManager {
        FileManager {
            filename: filename.as_ref().to_owned(),
            mode,
            file: None,
        }
    }

    fn with<F>(self: &mut Self, f: F) -> io::Result<()>
    where
        F: Fn(&mut File) -> io::Result<()>,
    {
        if self.mode == 'w' {
            self.file = Some(File::create(&self.filename)?);
            f(&mut self.file.as_mut().unwrap())?;
            self.file = None;
        }
        Ok(())
    }

    fn closed(self: &Self) -> bool {
        self.file.is_none()
    }
}

fn main() -> io::Result<()> {
    let mut fm = FileManager::new("test.txt", 'w');
    fm.with(|f| -> io::Result<()> {
        f.write(b"Test")?;
        Ok(())
    })?;
    println!("{}", fm.closed());
    Ok(())
}
