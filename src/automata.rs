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
        // Step 1: Mark equivalent state pairs (p, q) where p ∈ F and q ∉ F, or vice versa
        let mut equivalent_pairs: Vec<(String, String)> = Vec::new();
        for i in 0..self.states.len() {
            for j in (i + 1)..self.states.len() {
                let state1 = &self.states[i];
                let state2 = &self.states[j];
                if (self.final_states.contains(state1) && !self.final_states.contains(state2))
                    || (!self.final_states.contains(state1) && self.final_states.contains(state2))
                {
                    equivalent_pairs.push((state1.clone(), state2.clone()));
                }
            }
        }

        // Step 2: Initialize empty lists for each equivalent pair
        let mut equivalent_lists: Vec<Vec<(String, String)>> = vec![vec![]; equivalent_pairs.len()];

        // Step 3: Process transitions for equivalent pairs
        for i in 0..equivalent_pairs.len() {
            let (state1, state2) = (&equivalent_pairs[i].0, &equivalent_pairs[i].1);

            for symbol in &self.alphabet {
                let are_eq = self.are_equivalent(state1, state2, symbol, &equivalent_lists);

                if are_eq {
                    equivalent_lists[i].push((state1.clone(), state2.clone()));
                }
            }
        }

        // Step 4: Merge equivalent pairs
        for i in 0..equivalent_pairs.len() {
            if equivalent_lists[i].is_empty() {
                // These pairs are equivalent, so merge them.
                let (state1, state2) = (&equivalent_pairs[i].0, &equivalent_pairs[i].1);
                self.merge_states(state1, state2);
            }
        }
    }


    fn merge_states(&mut self, state1: &str, state2: &str) {
        // Remove state2 from the list of states
        self.states.retain(|state| state != state2);

        // Remove state2 from the list of final states
        self.final_states.retain(|state| state != state2);

        // Update transitions to use state1 instead of state2
        let updated_transitions: Vec<(String, String, String)> = self
            .transitions
            .iter()
            .map(|(from_state, symbol, to_state)| {
                if to_state == state2 {
                    (from_state.clone(), symbol.clone(), state1.to_string())
                } else {
                    (from_state.clone(), symbol.clone(), to_state.clone())
                }
            })
            .collect();
        self.transitions = updated_transitions;

        // Update start state if it was state2
        if self.start_state == state2 {
            self.start_state = state1.to_string();
        }
    }


    // A function to check if two states are equivalent based on your criteria
    fn are_equivalent(
        &self,
        state1: &str,
        state2: &str,
        symbol: &str,
        _equivalence_classes: &Vec<Vec<(String, String)>>
    ) -> bool {
        // Get transitions for state1 and state2 for the current symbol
        let transitions1 = self.get_state_transitions(state1, symbol);
        let transitions2 = self.get_state_transitions(state2, symbol);

        // Extract the destination states from transitions
        let destinations1: Vec<String> = transitions1.iter().map(|(_, _, to)| to.clone()).collect();
        let destinations2: Vec<String> = transitions2.iter().map(|(_, _, to)| to.clone()).collect();

        // Sort and compare the destination states
        let mut sorted_destinations1 = destinations1.clone();
        let mut sorted_destinations2 = destinations2.clone();

        sorted_destinations1.sort();
        sorted_destinations2.sort();

        sorted_destinations1 == sorted_destinations2
    }

    fn get_state_transitions(&self, state: &str, symbol: &str) -> Vec<(String, String, String)> {
        // Find and return transitions for the given state and symbol
        self.transitions
            .iter()
            .filter(|(from_state, sym, _)| from_state == state && sym == symbol)
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
