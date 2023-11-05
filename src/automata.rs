use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

pub struct DeterministicFinalAutomata {
    nr_of_states: usize,
    states: Vec<String>,
    alphabet: Vec<String>,
    start_state: String,
    final_states: Vec<String>,
    transitions: HashSet<(String, String, String)>,
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
            transitions: HashSet::new(),
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

    pub fn add_transition(&mut self, from_state: &str, symbols: &str, to_state: &str) {
        self.transitions.insert((from_state.to_string(), symbols.to_string(), to_state.to_string()));
    }

    pub fn minimize(&mut self) {
        // Step 1: Mark distinguishable state pairs (p, q) where p ∈ F and q ∉ F, or vice versa
        println!("Working on step 1...");
        let mut distinguishable_pairs: Vec<(String, String)> = Vec::new();
        for i in 0..(self.states.len() - 1) {
            for j in (i + 1)..(self.states.len() - 1) {
                let state1 = &self.states[i];
                let state2 = &self.states[j];
                // XOR for compactness - true if one of the states is final and the other is not
                // {q0, q1} is the same as {q1, q0} hence the double insertion - not the best solution, use alphabetic sorting instead
                if self.final_states.contains(state1) ^ self.final_states.contains(state2) {
                    distinguishable_pairs.push((state1.clone(), state2.clone()));
                    distinguishable_pairs.push((state2.clone(), state1.clone()));
                }
            }
        }

        // Step 2: Iterate through all state pairs and mark distinguishable pairs
        // For each state pair, there are symbols that they can transition on.
        // Using the symbol, we can get the destination states for each state pair.
        // If they are marked as distinguishable, then the original state pair (the source state pair) should also be marked distinguishable.
        // However, the state of being "distinguishable" should be propagated backwards.
        // This is why the algorithm should repeat until no more pairs can be marked as distinguishable.
        println!("Working on step 2...");
        let mut changed = true;
        let mut non_distinguishable_final: HashSet<(String, String)> = HashSet::new();

        while changed {
            changed = false;
            let mut non_distinguishable: HashSet<(String, String)> = HashSet::new();

            for i in 0..(self.states.len() - 1) {
                for j in (i + 1)..self.states.len() {
                    let state1 = &self.states[i];
                    let state2 = &self.states[j];

                    for symbol in &self.alphabet {
                        let transitions1 = self.get_state_transition(state1, symbol);
                        let transitions2 = self.get_state_transition(state2, symbol);
                        let transition = (transitions1.2.to_string(), transitions2.2.to_string());
                        let current_state = (state1.clone(), state2.clone());

                        if distinguishable_pairs.contains(&transition) && !distinguishable_pairs.contains(&current_state) {
                            // If the destination states are different, then the state pair is distinguishable
                            // {q0, q1} is the same as {q1, q0} hence the double insertion
                            distinguishable_pairs.push(current_state);
                            distinguishable_pairs.push((state2.clone(), state1.clone()));
                            changed = true;
                        } else if !distinguishable_pairs.contains(&current_state) {
                            // If the destination states are the same, then the state pair is NOT distinguishable
                            non_distinguishable.insert(current_state);
                        }
                    }
                }
            }
            non_distinguishable_final = non_distinguishable;
        }

        // Step 3. Merge all state pairs that are NOT distinguishable.
        // Use the merge_states function, as it should work properly.
        println!("Working on step 3...");

        // Create data structures for the new DFA
        let mut new_states: HashSet<String> = HashSet::new();
        let mut state_mapping: HashMap<String, String> = HashMap::new();
        let mut new_transitions: HashSet<(String, String, String)> = HashSet::new();
        let mut new_final_states: HashSet<String> = HashSet::new();
        let mut new_start_state: String = String::new();

        // Create a sorted list of non-distinguishable state pairs
        let mut non_distinguishable_sorted: Vec<(String, String)> = non_distinguishable_final.iter().cloned().collect();
        non_distinguishable_sorted.sort();

        // Process each non-distinguishable state pair
        for (state1, state2) in non_distinguishable_sorted {
            let merged_state = state1.clone() + &state2;
            new_states.insert(merged_state.clone());
            state_mapping.insert(state1.clone(), merged_state.clone());
            state_mapping.insert(state2.clone(), merged_state.clone());
            if self.final_states.contains(&state1) && self.final_states.contains(&state2) {
                new_final_states.insert(merged_state.clone());
            }
            if state1 == self.start_state || state2 == self.start_state {
                new_start_state = merged_state.clone();
            }
        }

        // Process remaining states
        for state in &self.states {
            if !state_mapping.contains_key(state) {
                state_mapping.insert(state.clone(), state.clone());
                new_states.insert(state.clone());
                if self.final_states.contains(state) {
                    new_final_states.insert(state.clone());
                }
                if state == &self.start_state {
                    new_start_state = state.clone();
                }
            }
        }

        // Update transitions with the merged states and sort them
        for (state, symbol, next_state) in &self.transitions {
            let state = state_mapping.get(state).unwrap();
            let next_state = state_mapping.get(next_state).unwrap();
            let new_transition = (state.clone(), symbol.clone(), next_state.clone());
            new_transitions.insert(new_transition);
        }

        // Sort the new states and transitions
        let mut new_states_vec: Vec<String> = new_states.iter().cloned().collect();
        new_states_vec.sort();

        // Note: The transitions sort somehow is not working properly, in every run it gives a different result, and I don't know why.
        let mut sorted_transitions: Vec<(String, String, String)> = new_transitions.iter().cloned().collect();
        sorted_transitions.sort();

        // Update the DFA
        self.nr_of_states = new_states.len();
        self.states = new_states_vec;
        self.final_states = new_final_states.iter().cloned().collect();
        self.start_state = new_start_state;

        // Replace the old transitions with the updated transitions
        self.transitions = sorted_transitions.iter().cloned().collect();
    }

    fn get_state_transition(&self, state: &str, symbol: &str) -> (String, String, String) {
        // Find and return transition for the given state and symbol
        self.transitions
            .iter()
            .filter(|(from_state, sym, _)| from_state == state && sym == symbol)
            .next()
            .cloned()
            .unwrap()
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

    let transitions: HashSet<(String, String, String)> = lines
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
        dfa.add_transition(&transition.0, &transition.1, &transition.2);
    }

    dfa
}
