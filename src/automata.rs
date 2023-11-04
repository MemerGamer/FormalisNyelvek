use std::fs::File;
use std::collections::HashSet;
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

    pub fn add_transition(&mut self, from_state: String, symbols: String, to_state: String) {
        self.transitions.insert((from_state, symbols, to_state));
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
        while changed {
            changed = false;
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
                        }
                    }
                }
            }
        }

        // Step 3. Merge all state pairs that are NOT distinguishable.
        // Use the merge_states function, as it should work properly.
        println!("Working on step 3...");

        for i in 0..(self.states.len() - 1) {
            let mut j = i + 1;

            while j < self.states.len() {
                let state1 = &self.states[i];
                let state2 = &self.states[j];

                if !distinguishable_pairs.contains(&(state1.clone(), state2.clone())) {
                    // Iterator is not advanced when merging, because the list of states shrinks by one, so the new state at the current index must also be checked
                    // The iterator of the for loop cannot be modified, which is why the while loop is needed
                    println!("Mergeable: {} and {}", state1, state2);
                    self.merge_states(state1.clone().as_str(), state2.clone().as_str());
                } else {
                    // Advance iterator if no merge was done
                    j += 1
                }
            }
        }
    }

    fn merge_states(&mut self, state1: &str, state2: &str) {
        // Remove state2 from the list of states
        self.states.retain(|state| state != state2);

        // Remove state2 from the list of final states
        self.final_states.retain(|state| state != state2);

        // Update transitions to use state1 instead of state2
        // Duplicate transitions can be eliminated by using a HashSet 
        let updated_transitions: HashSet<(String, String, String)> = self
            .transitions
            .iter()
            .map(|(from_state, symbol, to_state)| {
                // If A == B, then there are 3 cases which need to be accounted for:
                // B->C then B becomes A
                // C->B then B becomes A
                // A->C then A doesn't change
                if from_state == state2 {
                    (state1.to_string(), symbol.clone(), to_state.clone())
                } else if to_state == state2 {
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
        dfa.add_transition(transition.0, transition.1, transition.2);
    }

    dfa
}
