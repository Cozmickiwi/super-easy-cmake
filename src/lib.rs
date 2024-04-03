use std::{
    env::{current_dir, set_current_dir},
    fs::{create_dir, File},
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

const CMLCONTENTS: [&str; 10] = [
    "cmake_minimum_required(VERSION 3.10)",
    "project(MyProject)",
    "",
    "# Add source files",
    "set(SOURCES",
    "    src/main.c",
    ")",
    "",
    "# Add executable target",
    "add_executable(my_executable ${SOURCES})",
];

const MAINCONTENTS: [&str; 6] = [
    "#include <stdio.h>",
    "",
    "int main() {",
    "    printf(\"Hello, world!\\n\");",
    "    return 0;",
    "}",
];

pub fn build_main(name: &str) -> std::io::Result<()> {
    let start_dir = current_dir()?;
    create_dir(name)?;
    set_current_dir(name)?;
    let mut cmakelists = File::create("CMakeLists.txt")?;
    let exet = format!("add_executable({name} ${{SOURCES}})");
    let mut tick: usize = 0;
    for line in CMLCONTENTS {
        if tick == 9 {
            writeln!(cmakelists, "{}", &exet).expect("Could not write to file!");
        } else {
            writeln!(cmakelists, "{}", line).expect("Could not write to file!");
        }
        tick += 1;
    }
    create_dir("src")?;
    set_current_dir("src")?;
    let mut main_file = File::create("main.c")?;
    for line in MAINCONTENTS {
        writeln!(main_file, "{}", line).expect("Could not write to file!");
    }
    set_current_dir(start_dir)?;
    set_current_dir(name)?;
    let name2 = format!("../{name}/");
    let mut opt = Command::new("cmake")
        .arg(&name2)
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = opt.stdout.as_mut().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("{}", line?);
    }
    opt.wait()?;
    Ok(())
}
