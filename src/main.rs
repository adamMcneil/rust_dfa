use std::collections::HashMap;

#[derive(Debug)]
struct DFA {
    number_of_states: u32,
    alphabet: Vec<char>,
    transitions: Vec<HashMap<char, u32>>,
    start_state: u32,
    accept_states: Vec<u32>,
}

impl DFA {
    fn build_dfa (number_of_states: u32, alphabet: Vec<char>, transitions: Vec<HashMap<char, u32>>, start_state: u32, accept_states: Vec<u32>) -> Self{
        Self { number_of_states: number_of_states, alphabet: alphabet.clone(), transitions: transitions.clone(), start_state: start_state, accept_states: accept_states.clone() }
    }

    fn run (&self, input: Vec<char>) {
        let mut current_state: u32 = self.start_state;

        for symbol in input {
            current_state = self.transitions.get(current_state).get(symbol);
        }
    }

    fn print_states (&self) {
        let mut i: u32 = 0;
        while i < self.number_of_states {
            if i == self.start_state {
                print!(" ");
            }
            println!("{}", i);
            i = i + 1;
        }
    }
}



fn main() {
    let num_states: u32 = 2;
    let alphabet = vec!['a', 'b'];
    let mut transitions = vec![HashMap::new(), HashMap::new()];
    transitions[0].insert('a', 1);
    transitions[0].insert('b', 0);
    transitions[1].insert('a', 0);
    transitions[1].insert('b', 1);
    let start_state: u32 = 0;
    let accept_states = vec![0];

    let dfa: DFA = DFA::build_dfa(num_states, alphabet, transitions, start_state, accept_states);
    dfa.run(vec!['a', 'b']);

    // let machine: DFA = DFA::build_dfa(20, 14);
    // machine.print_states();

    // println!();

    // // let option_machine: Option<DFA> = Some(DFA::build_dfa(5, 4));
    // match option_machine {
    //     Option::None => (),
    //     Option::Some(machine) => machine.print_states(), 
    // }

} 

