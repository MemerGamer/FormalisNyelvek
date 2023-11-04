/*
Author: Kovács Bálint-Hunor
Informatika III.
*/

mod automata;

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

fn main() {
    test_minimization_algorithm();
}


