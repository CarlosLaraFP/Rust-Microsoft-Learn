// chrono is a good crate to use if you need to handle date and time data in Rust.
// It provides an easy API for representing a moment in time.

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Result, Seek, SeekFrom};


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// We annotated the created_at field, passing ts_seconds from chrono to the serde(with = ...)
// attribute so chrono can inform serde how its Datetime type will implement the two new traits

/*
    The question mark symbol (?) after the first statement is used to propagate errors without
    writing too much boilerplate code. It's syntax sugar for early returning an error if that error
    matches with the return type of the function it's in.
 */

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    /*
        Consume the file's contents as a vector of tasks.
        serde_json::Error can easily be converted to the std::io::Error type because it implements
        the From trait. That makes it possible for us to use the ? operator to unpack or early return them.
     */
    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(), // case with condition == guard
        Err(e) => Err(e)?,
    };

    /*
        Rewind the file after reading from it.
        Because we moved the cursor to the end of the file, we need to rewind the file before we
        write over it again. If we don't rewind the file, we'd begin writing at the cursor's last
        position, which would cause a malformed JSON file. We use the Seek trait and the SeekFrom
        enum from the std::io module to rewind the file.
     */
    file.seek(SeekFrom::Start(0))?;

    /*
        Write the modified task list back into the file.
        Finally, we push the Task value received as a function parameter to the task list and use
        serde_json to write the task vector into the file. We then return the empty tuple value
        inside an Ok to indicate that everything went according to our plans.
     */
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}
