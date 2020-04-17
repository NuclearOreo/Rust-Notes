use std::process::Command;

pub fn run() {
    // python demo.py
    let mut cmd = Command::new("python");
    cmd.arg("demo.py");

    match cmd.output() {
        Ok(o) => unsafe {
            let output = String::from_utf8_unchecked(o.stdout);
            println!("{}", output);
        },
        Err(e) => println!("There was an error: {}", e),
    }
}
