use std::process::Command;
use std::io::{self, Write};
use std::env;
use std::path::Path;


fn main() {
    // get the commandline args provided
    let args: Vec<String> = env::args().collect();

    // Ensure the arg is an existing file
    let src_file = Path::new(&args[1]);
    src_file.try_exists().expect(format!("Cannot find file: {}", src_file.display()).as_str());


    // Create a relative path using the name of the rust file we were passed initiall 
    let out_file_binding = Path::new(".").join(src_file.file_stem().unwrap());
    let out_file = out_file_binding.display();

    // Compile the rust oprogram and if it succeeds run the file created in the CWD
    let output = Command::new("rustc")
        .arg(src_file)
        .status()
        .and_then(|_| Command::new(format!("./{out_file}")).output())
        .expect("rustc or running the program has failed");

    // Pass along the outputs to our outputs so we can review
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    // make sure it succeeded
    assert!(output.status.success());
}
