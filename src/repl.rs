// I completely love this
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::Duration,
};

use crate::utils::read_line;

pub const END_TOKEN: &str = "~THE END~";

pub fn repl_launcher(name: &str) -> Box<PathBuf> {
    Box::new(Path::new("launchers/").join(name))
}

pub struct Repl {
    process: Child,
    pub exe: String,
}

impl Repl {
    pub fn spawn(exe: &str, dir: &str) -> Result<Self, String> {
        let launcher = match repl_launcher(exe).to_str() {
            Some(c) => String::from(c),
            None => {
                return Err(String::from(
                    "Internal error constructing shell launcher path",
                ));
            }
        };
        let mut resolvedir = dir.replace(
            "~",
            std::env::home_dir()
                .expect("Error finding home directory")
                .to_str()
                .unwrap(),
        );
        if resolvedir.eq(".") {
            resolvedir = std::env::current_dir()
                .expect("Error finding current directory")
                .to_str()
                .unwrap()
                .to_string();
        }
        let child = match Command::new(launcher)
            .env("LANG", "en_US.UTF-8")
            .env("LC_ALL", "en_US.UTF-8")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .current_dir(resolvedir)
            .spawn()
        {
            Ok(child) => child,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Self {
            process: child,
            exe: exe.to_string(),
        })
    }

    pub fn evaluate(&mut self, runtype: &str, txt: &str) -> Result<(), String> {
        if let Err(e) = writeln!(
            match &self.process.stdin {
                Some(s) => s,
                None => return Err(String::from("Child has no .stdin")),
            },
            "{END_TOKEN}\n{runtype}\n{txt}\n{END_TOKEN}"
        ) {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }
    pub fn kill(&self) {
        _ = writeln!(
            match &self.process.stdin {
                Some(s) => s,
                None => return,
            },
            "kill\n"
        );
        sleep(Duration::from_secs(5));
    }
}
impl Iterator for Repl {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Vec<u8>> {
        match &mut self.process.stdout {
            None => None,
            Some(stdout) => {
                let str = read_line(stdout)?;
                if str.eq(END_TOKEN.as_bytes()) {
                    None
                } else {
                    Some(str)
                }
            }
        }
    }
}
