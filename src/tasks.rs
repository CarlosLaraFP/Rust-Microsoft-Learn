// chrono is a good crate to use if you need to handle date and time data in Rust.
// It provides an easy API for representing a moment in time.

use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Result, Seek, SeekFrom, Error, ErrorKind}; // Include the `Error` type.
use std::fmt;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
/*
    We annotated the created_at field, passing ts_seconds from chrono to the serde(with = ...)
    attribute so chrono can inform serde how its Datetime type will implement the two new traits.
    We didn't define the created_at field by using the DateTime<Local> type from the beginning
    because the chrono::serde::ts_seconds module expects DateTime structs to be specialized over the Utc type.
 */

/*
    The question mark symbol (?) after the first statement is used to propagate errors without
    writing too much boilerplate code. It's syntax sugar for early returning an error if that error
    matches with the return type of the function it's in. These snippets are equivalent:

    fn function_1() -> Result(Success, Failure) {
        match operation_that_might_fail() {
            Ok(success) => success,
            Err(failure) => return Err(failure),
        }
    }

    fn function_2() -> Result(Success, Failure) {
        operation_that_might_fail()?
    }
 */

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    /*
        Consume the file's contents as a vector of tasks.
        serde_json::Error can easily be converted to the std::io::Error type because it implements
        the From trait. That makes it possible for us to use the ? operator to unpack or early return them.
     */
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    /*
        Rewind the file after reading from it.
        Because we moved the cursor to the end of the file, we need to rewind the file before we
        write over it again. If we don't rewind the file, we'd begin writing at the cursor's last
        position, which would cause a malformed JSON file. We use the Seek trait and the SeekFrom
        enum from the std::io module to rewind the file.
     */
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    /*
        Without the ? the variable type is Result<Vec<Task>>
        With the ? the variable type is Vec<Task>
        Same with the file binding above.
     */
    let mut tasks = collect_tasks(&file)?;
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

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks = collect_tasks(&file)?;

    // Remove the task.
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    tasks.remove(task_position - 1);

    /*
        Truncate the file.
        We're truncating the file before writing to it because we're performing a removal operation.
        So the file will be smaller than the original. If we ignored this step, the rewound cursor
        would stop behind the previously written bytes of the file, resulting in a malformed JSON
        file. When we truncate the file by using the file.set_len(0) operation, we ensure that
        we're writing the bytes in a blank page.
     */
    file.set_len(0)?;

    // Write the modified task list back into the file.
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file.
    let file = OpenOptions::new()
        .read(true)
        .open(journal_path)?;

    // Parse the file and collect the tasks.
    let tasks = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {:?}", order, task);
            order += 1;
        }
    }

    Ok(())
}
