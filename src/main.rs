/*
Author: Kovács Bálint-Hunor
Informatika III.
*/

mod automata;
mod regular_expressions;
mod stack;

fn test_minimization_algorithm() {
    // Reading automata from resources/dfa_1.txt
    let filename = "src/resources/dfa_1.txt";
    let mut automata = automata::read_automata(filename);

    // Printing the dfa
    automata.print_automata();

    println!();
    println!("Minimizing the dfa...");
    // Minimizing the dfa
    automata.minimize();

    println!();
    // Printing the minimized dfa
    automata.print_automata();
}

fn test_equivalence_check(){
    // Read dfa_5 and dfa_6
    let filename_1 = "src/resources/dfa_5.txt";
    let filename_2 = "src/resources/dfa_6.txt";
    let mut automata_1 = automata::read_automata(filename_1);
    let mut automata_2 = automata::read_automata(filename_2);

    // Print the automatas
    automata_1.print_automata();
    automata_2.print_automata();

    // Check if they are equivalent
    println!();
    println!("Checking if the automatas are equivalent...");
    let result = automata::check_equivalence(&mut automata_1, &mut automata_2);
    println!("The automatas are equivalent: {}", result);

}

fn test_regular_expressions(){
    let filename_1 = "src/resources/2022-10-15-10.18.37.log";
    if let Err(err) = regular_expressions::process_log_file(filename_1) {
        println!("Error: {}", err);
    }
}

fn test_stack_automata(){
    let filename_1 = "src/resources/dfa_7.txt";
    let word = String::from("aaabbb");
    let mut automata = stack::read_automata(filename_1);
    automata.print_automata();

    println!("Processing the word: {}", word);
    let result = automata.process_word(&word);
    println!("The word is accepted: {}", result);
}
fn main() {
    loop {
        println!("Which excercise would you like to run? [0(exit), 1 (dfa minimization), 2 (equivalence test), 3 (regex), 4 (stack-automata)]:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match input {
            0 => {
                println!("Exiting...");
                break;
            },
            1 => {
                println!("1. Testing the minimization algorithm");
                test_minimization_algorithm();
            },
            2 => {
                println!("2. Testing the equivalence check algorithm");
                test_equivalence_check();
            },
            3 => {
                println!("3. Testing the regular expressions");
                test_regular_expressions();
            },
            4 => {
                println!("4. Testing the stack automata");
                test_stack_automata();

            },
            _ => {
                println!("Invalid input, please try again");
                continue;
            }
        }
    }
}


