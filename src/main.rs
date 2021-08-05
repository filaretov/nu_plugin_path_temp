use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{CallInfo, ReturnSuccess, ReturnValue, Signature, UntaggedValue, Value};
use nu_source::Tag;

struct Len;

impl Len {
    fn new() -> Len {
        Len
    }

    fn temp_dir(&mut self) -> Result<Value, ShellError> {
        Ok(Value {
            value: UntaggedValue::filepath(Self::create_temp_dir()),
            tag: Tag::default(),
        })
    }

    fn create_temp_dir() -> PathBuf {
        let sys_temp = std::env::temp_dir();
        let new_temp_dir = loop {
            let now = Self::now();
            let subdir = format!("temp-dir-{}", now);
            let candidate = sys_temp.join(Path::new(&subdir));
            if !candidate.exists() {
                break candidate;
            }
        };
        create_dir(&new_temp_dir).expect("Failed to create directory");
        new_temp_dir
    }

    fn now() -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Is your system time before the Unix epoch?")
            .as_millis()
            .to_string()
    }
}

impl Plugin for Len {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("path temp")
            .desc("Create a directory in the system's temporary directory")
            .filter())
    }

    fn begin_filter(&mut self, _: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![ReturnSuccess::value(self.temp_dir()?)])
    }

    fn filter(&mut self, _: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![])
    }
}

fn main() {
    serve_plugin(&mut Len::new());
}
