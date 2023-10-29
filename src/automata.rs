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
        // Step 1: Mark distinguishable state pairs (p, q) where p ∈ F and q ∉ F, or vice versa
        let mut distinguishable_pairs: Vec<(String, String)> = Vec::new();
        for i in 0..(self.states.len() - 1) {
            for j in (i + 1)..self.states.len() {
                let state1 = &self.states[i];
                let state2 = &self.states[j];
                // XOR for compactness - true if one of the states is final and the other is not
                if self.final_states.contains(state1) ^ self.final_states.contains(state2) {
                    distinguishable_pairs.push((state1.clone(), state2.clone()));
                }
            }
        }

        // Step 2: Iterate through all state pairs and mark distinguishable pairs
        // For each state pair, there are symbols that they can transition on.
        // Using the symbol, we can get the destination states for each state pair.
        // If they are marked as distinguishable, then the original state pair (the source state pair) should also be marked distinguishable.
        // However, the state of being "distinguishable" should be propagated backwards.
        // This is why the algorithm should repeat until no more pairs can be marked as distinguishable.
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..(self.states.len() - 1) {
                for j in (i + 1)..self.states.len() {
                    let state1 = &self.states[i];
                    let state2 = &self.states[j];
                    for symbol in &self.alphabet {
                        let transitions1 = self.get_state_transitions(state1, symbol);
                        let transitions2 = self.get_state_transitions(state2, symbol);

                        // Extract the destination states from transitions
                        let destinations1: Vec<String> = transitions1
                            .iter()
                            .map(|(_, _, to)| to.clone())
                            .collect();
                        let destinations2: Vec<String> = transitions2
                            .iter()
                            .map(|(_, _, to)| to.clone())
                            .collect();

                        // Sort and compare the destination states
                        let mut sorted_destinations1 = destinations1.clone();
                        let mut sorted_destinations2 = destinations2.clone();

                        sorted_destinations1.sort();
                        sorted_destinations2.sort();

                        if sorted_destinations1 != sorted_destinations2 {
                            // If the destination states are different, then the state pair is distinguishable
                            if !distinguishable_pairs.contains(&(state1.clone(), state2.clone())) {
                                distinguishable_pairs.push((state1.clone(), state2.clone()));
                                changed = true;
                            }
                        } else {
                            // If the destination states are the same, then the state pair is not distinguishable
                            if distinguishable_pairs.contains(&(state1.clone(), state2.clone())) {
                                distinguishable_pairs.retain(|pair| pair != &(state1.clone(), state2.clone()));
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        // Step 3. Merge all state pairs that are NOT distinguishable.
        // Use the merge_states function, as it should work properly.
        let mut equivalent_classes: Vec<Vec<String>> = vec![vec![]];
        for state in &self.states {
            let mut added = false;
            for class in equivalent_classes.iter_mut() {
                if class.is_empty()
                    || !distinguishable_pairs.contains(&(class[0].clone(), state.clone()))
                {
                    class.push(state.clone());
                    added = true;
                    break;
                }
            }
            if !added {
                equivalent_classes.push(vec![state.clone()]);
            }
        }

        for class in equivalent_classes.iter() {
            if class.len() > 1 {
                for i in 1..class.len() {
                    self.merge_states(&class[0], &class[i]);
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
        let updated_transitions: Vec<(String, String, String)> = self
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
