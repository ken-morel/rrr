use std::{cell::RefMut, io::{BufRead, BufReader, Read, Write}, process::{Child, ChildStdin, ChildStdout}};

use super::eres;

pub type SpawnResult<T> = Result<T, String>;

pub const END_TOKEN: &str = "~THE END~";

pub trait Repl<R> {
    fn new(stdin: ChildStdin, stdout:  ChildStdout) -> R;
    fn streams<'a>(&'a self) -> (RefMut<'a, ChildStdin>, RefMut<'a, ChildStdout>) ;
    fn parse_eval_result(&self, txt: String) -> eres::EvalResult;
    fn evaluate(&self,runtype: &str , txt: &str) -> Result<eres::EvalResult, String> {
        let (mut input, mut output) = self.streams();
        let mut reader = BufReader::new(output.by_ref());

        writeln!(input, "{END_TOKEN}\n{runtype}\n{txt}\n{END_TOKEN}").expect("Error sending message to child process");

        let mut ln = String::new();
        let mut output = String::new();
        loop {
            ln.clear();
            match reader.read_line(&mut ln) {
                Err(err) => return Err(err.to_string()),
                Ok(_) =>  {
                    if std::cmp::Ordering::Equal == ln.trim().cmp(END_TOKEN) {
                        break;
                    } else {
                        output += ln.as_str();
                    }
                },
            };
        }
        Ok(self.parse_eval_result(output))
    }
    fn kill(&self) {
        let (mut input, _) = self.streams();
        _ = writeln!(input, "kill\n");
    }
    
    
    fn from_child(child: Child) -> Result<R, String> {
        if let Some(stdin) =  child.stdin && let Some(stdout) = child.stdout {
            // FIX: wanted to put child here, but lsp talks about some partial
            // move crap. :-(
            return Ok(Self::new(stdin,  stdout));
        } else {
            return Err(String::from("Error getting spawned child input and output streams"));
        }

    }
    
   

}
