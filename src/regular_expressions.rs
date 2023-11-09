use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};


pub fn process_log_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Open or create the processed.txt file
    let mut processed_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("processed.txt")?;

    // Task 1
    let re_task1 = Regex::new(r"(\d{2}-10-15 \d{2}:\d{2}:\d{2}\.\d{3} \d+ \d+ [IDVEFAW])").unwrap();

    // Task 2
    let re_task2 = Regex::new(r"(\d{2}-10-15 10:18:51\.\d{3} \d+ \d+ [IDVEFAW])").unwrap();

    // Task 3
    let re_task3 = Regex::new(r"(\d{2}-\d{2}-\d{2} \d{2}:\d{2}:51\.\d{3} \d+ \d+ [IDVEFAW])").unwrap();

    // Task 4
    let re_task4 = Regex::new(r"([IDVEFAW].*lowmemorykiller.*)").unwrap();

    // Task 5
    let re_task5 = Regex::new(r"(\d{2}-\d{2}-\d{2} 10:18:51\.\d{3} \d+ 221 [IDVEFAW])").unwrap();

    // Task 6
    let re_task6 = Regex::new(r"(\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} \d+ \d+ E [^:]+: .*)").unwrap();

    // Task 7
    let re_task7 = Regex::new(r"(\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} \d+ \d+ W PackageManager: .*)").unwrap();

    // Task 8
    let re_task8 = Regex::new(r"(\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} \d+ \d+ D [^:]+: .*)").unwrap();

    // Task 9
    let re_task9 = Regex::new(r"(\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} \d+ \d+ [EW] [^:]+\.java: .*)").unwrap();

    // Task 10
    let re_task10 = Regex::new(r"(\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} \d+ \d+ [IDVEFAW] [^:]+: .*Thread.*)").unwrap();

    // Additional tasks
    let re_stack_trace = Regex::new(r"(\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} \d+ \d+ E [^:]+: .*)").unwrap();
    let re_process_id = Regex::new(r"\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3} (\d+) \d+ [IDVEFAW]").unwrap();

    let mut process_errors_count = std::collections::HashMap::new();

    // Process each line in the log file
    for line in reader.lines() {
        let line = line?;
        if re_task6.is_match(&line) {
            writeln!(processed_file, "Task 6: {}", line)?;

            // Additional Task 1: Print the origin of lines with errors (StackTrace)
            if let Some(captures) = re_stack_trace.captures(&line) {
                writeln!(processed_file, "StackTrace: {}", captures.get(1).unwrap().as_str())?;
            }

            // Additional Task 2: Print distinct process IDs
            if let Some(captures) = re_process_id.captures(&line) {
                let process_id = captures.get(1).unwrap().as_str();
                writeln!(processed_file, "Distinct Process ID: {}", process_id)?;

                // Additional Task 3: Count the process errors
                let count = process_errors_count.entry(process_id.to_string()).or_insert(0);
                *count += 1;
            }
        }
    }

    // Additional Task 4: Print the process with the most errors
    if let Some((process_id, count)) = process_errors_count.iter().max_by(|&(_, a), &(_, b)| a.cmp(b)) {
        writeln!(processed_file, "Process with the most errors: {} ({} errors)", process_id, count)?;
    }

    // Additional Task 5: Print errors on the main thread
    for (process_id, count) in process_errors_count.iter() {
        if process_id == "thread_id_here" {
            writeln!(processed_file, "Errors on the main thread: {} ({} errors)", process_id, count)?;
        }
    }

    Ok(())
}
