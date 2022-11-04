use std::{
    fs::{self, File},
    io::{BufReader, Read, Result, Write},
    path::Path,
};

fn main() -> Result<()> {
    let path = "test.md";
    if !Path::new(path).exists() {
        let mut file = File::create(path)?;
        let content = r#"# Head 1

first head1 content.

## Head2

second head2 content."#;
        file.write_all(content.as_bytes())?;
    }
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("file content: {}", contents);

    // read file line by line
    // for line in buf_reader.lines() {
    //     println!("{}", line?);
    // }

    fs::remove_file(path)?;
    // remove specified directory and contents in it.
    // fs::remove_dir_all("ttt")?;

    Ok(())
}
