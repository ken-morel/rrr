// I completely love this
use {
    std::{
        cell::{
            RefCell,
            RefMut,
        },
        io::{
            BufRead,
            BufReader,
            Read,
            Write,
        },
        path::{
            Path,
            PathBuf,
        },
        process::{
            Child,
            ChildStdin,
            ChildStdout,
            Command,
            Stdio,
        },
        thread::sleep,
        time::Duration,
    },
    super::eres
};

pub const END_TOKEN: &str = "~THE END~";

pub fn repl_launcher(name: &str) -> Box<PathBuf> {
    Box::new(Path::new("launchers/").join(name))
}

pub struct Repl {
    stdout: RefCell<ChildStdout>,
    stdin: RefCell<ChildStdin>,
}

impl Repl {
    pub fn spawn(exe: &str, dir: &str) -> Result<Self, String> {
        let launcher = match repl_launcher(exe).to_str() {
            Some(c) => String::from(c),
            None => {
                return Err(String::from(
                    "Internal error constructing shell launcher path",
                ))
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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .current_dir(resolvedir)
            .spawn()
        {
            Ok(child) => child,
            Err(err) => return Err(err.to_string()),
        };
        Repl::from_child(child)
    }
    pub fn from_child(child: Child) -> Result<Self, String> {
        if let Some(stdin) =  child.stdin && let Some(stdout) = child.stdout {
            // FIX: wanted to put child here, but lsp talks about some partial
            // move crap. :-(
            return Ok(Self {
                stdin: RefCell::new(stdin),
                stdout: RefCell::new(stdout),
            })
        } else {
            return Err(String::from("Error getting spawned child input and output streams"));
        }

    }
    pub fn streams(&self) -> (RefMut<'_, ChildStdin>, RefMut<'_, ChildStdout>) {
        (self.stdin.borrow_mut(), self.stdout.borrow_mut())
    }

    pub fn parse_eval_result(&self, txt: String) -> eres::EvalResult {
        if txt.starts_with("ERRROR") {
            eres::EvalResult::Error(txt)
        } else {
            eres::EvalResult::Text(txt)
        }
    }
    pub fn evaluate(&self,runtype: &str , txt: &str) -> Result<eres::EvalResult, String> {
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
    pub fn kill(&self) {
        let (mut input, _) = self.streams();
        _ = writeln!(input, "kill\n");
        sleep(Duration::from_secs(5));
    }
}
