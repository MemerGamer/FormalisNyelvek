use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead};

pub struct StackAutomata {
    nr_of_states: usize,
    states: Vec<String>,
    alphabet: Vec<String>,
    stack_alphabet: Vec<String>,
    start_state: String,
    stack_start: String,
    final_states: Vec<String>,
    transitions: Vec<(String, String, String, String, String)>,
    stack: VecDeque<String>,
}

impl StackAutomata {
    fn new(nr_of_states: usize, states: Vec<String>, alphabet: Vec<String>, stack_alphabet: Vec<String>,
           start_state: String, stack_start: String, final_states: Vec<String>) -> Self {
        StackAutomata {
            nr_of_states,
            states,
            alphabet,
            stack_alphabet,
            start_state,
            stack_start,
            final_states,
            transitions: Vec::new(),
            stack: VecDeque::new(),
        }
    }

    fn add_transition(&mut self, from_state: String, input_symbol: String,
                      stack_symbol: String, new_stack_symbols: String, to_state: String) {
        self.transitions.push((from_state, input_symbol, stack_symbol, new_stack_symbols, to_state));
    }

    fn add_to_stack(&mut self, new_stack_symbols: &str) {
        self.stack.push_back(new_stack_symbols.to_string());

        // Print the current state of the stack
        println!("Stack after adding: {:?}", self.stack);
    }

    fn remove_from_stack(&mut self) {
        // Pop a symbol from the stack
        if let Some(popped_symbol) = self.stack.pop_back() {
            // Print the symbol that was removed and the current state of the stack
            println!("Removed symbol: {}", popped_symbol);
            println!("Stack after removing: {:?}", self.stack);
        }
    }

    fn is_final_state(&self, state: &str) -> bool {
        self.final_states.contains(&state.to_string())
    }

    fn is_valid_transition(&self, from_state: &str, input_symbol: &str,
                           stack_state: &str) -> bool {
        self.transitions.iter().any(|(transition_from_state, transition_input_symbol,
                                         transition_stack_symbol, _, _)| {
            transition_from_state == from_state
                && transition_input_symbol == input_symbol
                && transition_stack_symbol.starts_with(stack_state)
        })
    }

    fn get_transition(&self, from_state: &str, input_symbol: &str, stack_state: &str)
                      -> Option<(String, String, String, String, String)> {
        self.transitions.iter().find(|transition|
            transition.0 == from_state && transition.1 == input_symbol && transition.2 == stack_state
        ).cloned()
    }

    pub(crate) fn process_word(&mut self, word: &str) -> bool {
        // Add the start symbol to the stack
        let stack_start = self.stack_start.clone();
        self.add_to_stack(&stack_start);

        let mut current_state = self.start_state.clone();
        let mut current_stack_state = stack_start.clone();

        for letter in word.chars() {
            if let Some(letter_transition) = self.get_transition(&current_state, &letter.to_string(), &current_stack_state) {
                println!("Current transition: {:?}", letter_transition);

                let (_, _, _, next_stack_state, next_state) = letter_transition;

                let second_part = if next_stack_state.chars().next().unwrap() != 'E' {
                    next_stack_state[2..].to_string()
                } else {
                    "E".to_string()
                };

                let next_stack_state = second_part;

                if self.is_valid_transition(&current_state, &letter.to_string(), &current_stack_state) {
                    if next_stack_state == "E" {
                        self.remove_from_stack();
                        current_state = next_state;
                        current_stack_state = self.stack.back().map_or_else(|| stack_start.clone(), |s| s.clone());
                    } else {
                        self.add_to_stack(&next_stack_state);
                        current_state = next_state;
                        current_stack_state = next_stack_state.clone();
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        if self.is_final_state(&self.start_state) && self.stack.back().map_or(false, |s| s == &stack_start) {
            true
        } else {
            false
        }
    }

    pub(crate) fn print_automata(&self) {
        println!("The stack automata:");
        println!("Number of states: {}", self.nr_of_states);
        println!("States: {:?}", self.states);
        println!("Alphabet: {:?}", self.alphabet);
        println!("Stack alphabet: {:?}", self.stack_alphabet);
        println!("Start State: {}", self.start_state);
        println!("Stack start: {}", self.stack_start);
        println!("Final States: {:?}", self.final_states);
        println!("Transitions:");
        for (from_state, input_symbol, stack_symbol, new_stack_symbols, to_state) in &self.transitions {
            let second_part = if new_stack_symbols.chars().next().unwrap() != 'E' {
                new_stack_symbols[2..].to_string()
            } else {
                "E".to_string()
            };
            println!("{} --{}--> {} \n {} -> {}", from_state, input_symbol, to_state, stack_symbol, second_part);
        }
    }
}

pub fn read_automata(filename: &str) -> StackAutomata {
    let file = File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().map(|line| line.unwrap());

    // Read the states and alphabet
    let states_line = lines.next().unwrap();
    let states: Vec<String> = states_line.split_whitespace().map(|s| s.to_string()).collect();
    let alphabet: Vec<String> = lines.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
    let stack_alphabet: Vec<String> = lines.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();

    let nr_of_states = states.len();
    let start_state = lines.next().unwrap();
    let stack_start = lines.next().unwrap();
    let final_states: Vec<String> = lines.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
    let transitions: Vec<(String, String, String, String, String)> = lines.map(|line| {
        let parts: Vec<_> = line.split_whitespace().map(|s| s.to_string()).collect();
        (parts[0].clone(), parts[1].clone(), parts[2].clone(), parts[3].clone(), parts[4].clone())
    }).collect();

    let mut stack_automaton = StackAutomata::new(nr_of_states, states, alphabet, stack_alphabet, start_state, stack_start, final_states);

    for transition in transitions {
        stack_automaton.add_transition(
            transition.0.clone(), transition.1.clone(), transition.2.clone(), transition.3.clone(), transition.4.clone()
        );
    }

    stack_automaton
}
