use super::{eres, repl};
use std::{
    cell::{RefCell, RefMut},
    path::{Path, PathBuf},
    process::{ChildStdin, ChildStdout, Command, Stdio},
}; // from rust by example

pub fn repl_launcher(name: &str) -> Box<PathBuf> {
    Box::new(Path::new("/home/engon/apps/rrr/launchers/").join(name))
}

pub struct SimpleRepl {
    stdout: RefCell<ChildStdout>,
    stdin: RefCell<ChildStdin>,
}

impl repl::Repl<SimpleRepl> for SimpleRepl {
    // did it!!! and in less than 10 secs :eyeglasses:
    fn new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        Self {
            // I first tried with Rc :D
            stdin: RefCell::new(stdin),
            stdout: RefCell::new(stdout),
        }
    }
    fn streams(&self) -> (RefMut<'_, ChildStdin>, RefMut<'_, ChildStdout>) {
        (self.stdin.borrow_mut(), self.stdout.borrow_mut())
    }

    fn parse_eval_result(&self, txt: String) -> eres::EvalResult {
        if txt.starts_with("ERRROR") {
            eres::EvalResult::Error(txt)
        } else {
            eres::EvalResult::Text(txt)
        }
    }
}

impl SimpleRepl
where
    SimpleRepl: repl::Repl<SimpleRepl>,
{
    pub fn spawn(exe: &str) -> repl::SpawnResult<Self> {
        let launcher = match repl_launcher(exe).to_str() {
            Some(c) => String::from(c),
            None => {
                return Err(String::from(
                    "Internal error constructing shell launcher path",
                ))
            }
        };
        let child = match Command::new(launcher)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(child) => child,
            Err(err) => return Err(err.to_string()),
        };
        // 4mins of trial and error, playing with lsp
        <SimpleRepl as repl::Repl<SimpleRepl>>::from_child(child)
    }
}
