use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DeterministicFinalAutomata {
    nr_of_states: usize,
    states: Vec<String>,
    alphabet: Vec<String>,
    start_state: String,
    final_states: Vec<String>,
    transitions: Vec<(String, String, String)>,
}

impl DeterministicFinalAutomata {
    pub fn new(
        nr_of_states: usize,
        states: Vec<String>,
        alphabet: Vec<String>,
        start_state: String,
        final_states: Vec<String>,
    ) -> Self {
        Self {
            nr_of_states,
            states,
            alphabet,
            start_state,
            final_states,
            transitions: Vec::new(),
        }
    }

    pub fn print_automata(&self) {
        println!("The deterministic final automata:");
        println!("Number of states: {}", self.nr_of_states);
        println!("States: {:?}", self.states);
        println!("Alphabet: {:?}", self.alphabet);
        println!("Start State: {}", self.start_state);
        println!("Final States: {:?}", self.final_states);
        println!("Transitions:");
        for (from_state, symbols, to_state) in &self.transitions {
            println!("{} --{}--> {}", from_state, symbols, to_state);
        }
    }

    pub fn add_transition(&mut self, from_state: String, symbols: String, to_state: String) {
        self.transitions.push((from_state, symbols, to_state));
    }

    pub fn minimize(&mut self) {
        // Creating equivalence classes
        let mut equivalence_classes: Vec<Vec<String>> = Vec::new();

        // Initializing equivalence classes
        let final_states: Vec<String> = self.states.clone();
        let non_final_states: Vec<String> = self.states.iter()
            .filter(|&state| !final_states.contains(state))
            .cloned()
            .collect();
        equivalence_classes.push(final_states);
        equivalence_classes.push(non_final_states);

        // Refining equivalence classes
        let mut equivalence_classes_changed: bool = true;
        while equivalence_classes_changed {
            equivalence_classes_changed = false;
            let mut new_equivalence_classes: Vec<Vec<String>> = Vec::new();
            for equivalence_class in &equivalence_classes {
                let mut new_equivalence_class: Vec<String> = Vec::new();
                let mut equivalence_class_changed: bool = true;
                while equivalence_class_changed {
                    equivalence_class_changed = false;
                    for state in equivalence_class.iter() { // Use .iter() to get a reference
                        let mut is_new_state = true;
                        for new_state in &new_equivalence_class {
                            if self.are_equivalent(state, new_state, &equivalence_classes) {
                                is_new_state = false;
                                break;
                            }
                        }
                        if is_new_state {
                            new_equivalence_class.push(state.clone());
                            equivalence_class_changed = true;
                            equivalence_classes_changed = true;
                        }
                    }
                }
                new_equivalence_classes.push(new_equivalence_class);
            }
            equivalence_classes = new_equivalence_classes;
        }

        // Minimize the transitions
        let mut new_transitions: Vec<(String, String, String)> = Vec::new();
        for equivalence_class in equivalence_classes {
            let new_from_state: String = equivalence_class[0].clone();
            for state in &equivalence_class {
                for (from_state, symbols, to_state) in &self.transitions {
                    if from_state == state {
                        new_transitions.push((new_from_state.clone(), symbols.clone(), to_state.clone()));
                    }
                }
            }
        }

        // Update start and final states
        let mut new_start_state: String = self.start_state.clone();
        let mut new_final_states: Vec<String> = Vec::new();
        for equivalence_class in equivalence_classes {
            for state in &equivalence_class {
                if state == &self.start_state {
                    new_start_state = equivalence_class[0].clone();
                }
                for final_state in &self.final_states {
                    if state == final_state {
                        new_final_states.push(equivalence_class[0].clone());
                    }
                }
            }
        }

        // Remove redundant states
        let mut new_states: Vec<String> = Vec::new();
        for equivalence_class in equivalence_classes {
            new_states.push(equivalence_class[0].clone());
        }

        // Update the automata
        self.states = new_states;
        self.start_state = new_start_state;
        self.final_states = new_final_states;
        self.transitions = new_transitions;
    }

    // A function to check if two states are equivalent based on your criteria
    fn are_equivalent(&self, state1: &str, state2: &str, equivalence_classes: &Vec<Vec<String>>) -> bool {
        // Get transitions for state1 and state2
        let transitions1 = self.get_state_transitions(state1);
        let transitions2 = self.get_state_transitions(state2);

        // Compare transitions
        transitions1 == transitions2
    }

    fn get_state_transitions(&self, state: &str) -> Vec<(String, String, String)> {
        // Find and return transitions for the given state
        self.transitions
            .iter()
            .filter(|(from_state, _, _)| from_state == state)
            .cloned()
            .collect()
    }
}

// Function to read automata from a file and create an instance of DeterministicFinalAutomata
pub fn read_automata(filename: &str) -> DeterministicFinalAutomata {
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.expect("Error reading line"));

    let nr_of_states: usize = lines.next().unwrap().trim().parse().expect("Error parsing number of states");
    let states: Vec<String> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect();
    let alphabet: Vec<String> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect();
    let start_state: String = lines.next().unwrap().trim().to_string();
    let final_states: Vec<String> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect();

    let transitions: Vec<(String, String, String)> = lines
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
            )
        })
        .collect();

    let mut dfa = DeterministicFinalAutomata::new(nr_of_states, states, alphabet, start_state, final_states);

    for transition in transitions {
        dfa.add_transition(transition.0, transition.1, transition.2);
    }

    dfa
}
