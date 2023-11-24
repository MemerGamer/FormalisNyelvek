use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub fn process_log_file(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the content of the log file into a String
    let logcat: String = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");

    // I.
    // Task 1
    let pattern1 = Regex::new(r"^10-15").unwrap();
    let mut count10_15 = 0;

    // Task 2
    let pattern2 = Regex::new(r"^10-15 10:18:51").unwrap();
    let mut count10_15_10_18_51 = 0;

    // Task 3
    let pattern3 = Regex::new(r"^[0-9-]+ [0-9:]+:51").unwrap();
    let mut count51 = 0;

    // Task 4
    let pattern4 = Regex::new(r"lowmemorykiller").unwrap();
    let mut count_low_memory_killer = 0;

    // Task 5
    let pattern5 = Regex::new(r"^10-15 10:18:51.* 221 ").unwrap();
    let mut count10_18_51_221 = 0;

    // Task 6
    let pattern6 = Regex::new(r".* E ").unwrap();
    let mut count_e = 0;

    // Task 7
    let pattern7 = Regex::new(r".* W PackageManager.* ").unwrap();
    let mut count_package_manager_w = 0;

    // Task 8
    let pattern8 = Regex::new(r".* D .*ExoPlayer.* ").unwrap();
    let mut count_exo_player_d = 0;

    // Task 9
    let pattern9 = Regex::new(r".* [EW].*\.java:").unwrap();
    let mut count_ew_java = 0;

    // Task 10
    let pattern10 = Regex::new(r".* [^:]+: .*Thread.*").unwrap();
    let mut count_thread = 0;

    // Counting
    for line in logcat.lines() {
        if pattern1.is_match(line) {
            count10_15 += 1;
        }

        if pattern2.is_match(line) {
            count10_15_10_18_51 += 1;
        }

        if pattern3.is_match(line) {
            count51 += 1;
        }

        if pattern4.is_match(line) {
            count_low_memory_killer += 1;
        }

        if pattern5.is_match(line) {
            count10_18_51_221 += 1;
        }

        if pattern6.is_match(line) {
            count_e += 1;
        }

        if pattern7.is_match(line) {
            count_package_manager_w += 1;
        }

        if pattern8.is_match(line) {
            count_exo_player_d += 1;
        }

        if pattern9.is_match(line) {
            count_ew_java += 1;
        }

        if pattern10.is_match(line) {
            count_thread += 1;
        }
    }

    // Writing the results to the screen
    println!("Number of lines made on 10-15: {}", count10_15);
    println!("Number of lines made on 10-15 10:18:51: {}", count10_15_10_18_51);
    println!("Number of lines made on 51 seconds: {}", count51);
    println!("Number of lines containing lowmemorykiller: {}", count_low_memory_killer);
    println!("Number of lines made on 10-15 10:18:51 221: {}", count10_18_51_221);
    println!("Number of lines containing E: {}", count_e);
    println!("Number of lines containing PackageManager W: {}", count_package_manager_w);
    println!("Number of lines containing ExoPlayer D: {}", count_exo_player_d);
    println!("Number of lines containing E or W and .java: {}", count_ew_java);
    println!("Number of lines containing Thread: {}", count_thread);

    // Writing the results to the processed.txt file
    let mut processed_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("processed.txt")?;
    writeln!(processed_file, "Number of lines made on 10-15: {}", count10_15)?;
    writeln!(processed_file, "Number of lines made on 10-15 10:18:51: {}", count10_15_10_18_51)?;
    writeln!(processed_file, "Number of lines made on 51 seconds: {}", count51)?;
    writeln!(processed_file, "Number of lines containing lowmemorykiller: {}", count_low_memory_killer)?;
    writeln!(processed_file, "Number of lines made on 10-15 10:18:51 221: {}", count10_18_51_221)?;
    writeln!(processed_file, "Number of lines containing E: {}", count_e)?;
    writeln!(processed_file, "Number of lines containing PackageManager W: {}", count_package_manager_w)?;
    writeln!(processed_file, "Number of lines containing ExoPlayer D: {}", count_exo_player_d)?;
    writeln!(processed_file, "Number of lines containing E or W and .java: {}", count_ew_java)?;
    writeln!(processed_file, "Number of lines containing Thread: {}", count_thread)?;


    // II.
    let pattern_ex = Regex::new(r"([0-9-]+ [0-9:.]+) +([0-9]+) +([0-9]+) ([IDVEFAW]) ([^:]+): (.*)").unwrap();

    // Task 1
    let mut stack_traces = Vec::new();
    let mut current_stack_trace = String::new();
    let mut inside_stack_trace = false;

    for line in logcat.lines() {
        if let Some(captures) = pattern_ex.captures(line) {
            if captures[4].eq("E") {
                inside_stack_trace = true;
                current_stack_trace.push_str(&format!("{}:\n→ {}\n", captures[6].trim(), captures[6].trim()));
            }
        } else if inside_stack_trace && !line.starts_with(' ') {
            inside_stack_trace = false;
            stack_traces.push(current_stack_trace.clone());
            current_stack_trace.clear();
        }
    }

    for stack_trace in stack_traces {
        println!("{}", stack_trace);
        writeln!(processed_file, "{}", stack_trace)?;
    }

    // Task 2
    let unique_process_ids: Vec<_> = pattern_ex.captures_iter(logcat.as_str())
        .map(|cap| cap[3].to_string())
        .collect();
    let unique_process_ids: Vec<_> = unique_process_ids.into_iter().collect();
    println!("Unique process ids: {:?}", unique_process_ids);
    writeln!(processed_file, "Unique process ids: {:?}", unique_process_ids)?;

    // Task 3
    let error_processes: Vec<_> = pattern_ex.captures_iter(logcat.as_str())
        .filter(|cap| cap[4].eq("E"))
        .map(|cap| cap[3].to_string())
        .collect();
    let most_common_error_process = error_processes.iter()
        .max_by(|&x, &y| error_processes.iter().filter(|&e| e.clone() == x.clone()).count()
            .cmp(&error_processes.iter().filter(|&e| e.clone() == y.clone()).count()))
        .unwrap();
    println!("Most common error process: {}", most_common_error_process);
    writeln!(processed_file, "Most common error process: {}", most_common_error_process)?;

    // Task 4
    let main_thread_errors: Vec<_> = pattern_ex.captures_iter(logcat.as_str())
        .filter(|cap| cap[3] == cap[4] && cap[4].eq( "E"))
        .map(|cap| format!("{}: {}", cap[3].to_string(), cap[0].to_string()))
        .collect();
    println!("Main thread errors:");
    writeln!(processed_file, "Main thread errors:")?;
    for main_thread_error in main_thread_errors {
        println!("{}", main_thread_error);
        writeln!(processed_file, "{}", main_thread_error)?;
    }





    Ok(())
}

fn main() {
    let file_path = std::env::args().nth(1).expect("No filename given!");
    process_log_file(&file_path).expect("Error processing log file");
}
