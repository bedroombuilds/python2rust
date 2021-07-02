use std::io;
use std::path::{Path, PathBuf};

fn bytes_from_file(filename: impl AsRef<std::path::Path>) -> io::Result<Vec<u8>> {
    Ok(std::fs::read(filename)?)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let data = bytes_from_file(&args[1])?;
    println!("data size {}bytes", data.len());

    let user_home: PathBuf = dirs::home_dir().expect("Could not find home directory.");
    let user_fonts = Path::new(&user_home).join("Library").join("Fonts");
    println!("user_home: {:?} user_fonts: {:?}", user_home, user_fonts);

    let _data = bytes_from_file(user_fonts)?;
    Ok(())
}
