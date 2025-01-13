use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

use crate::aoc_solution::Solution;

pub struct Day24;

#[derive(Debug)]
enum WireState {
    On,
    Off,
}

impl WireState {
    fn from_str(value: &str) -> WireState {
        match value {
            "1" => WireState::On,
            "0" => WireState::Off,
            unknown => panic!("Unknown state: {}", unknown),
        }
    }
}

struct Wire {
    name: String,
    state: Option<WireState>,
    dependency_indices: Vec<usize>,
}

enum GateType {
    And,
    Or,
    Xor,
}

impl GateType {
    fn from_str(value: &str) -> GateType {
        match value {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            unknown => panic!("Unknown gate type: {}", unknown),
        }
    }
}

struct LogicGate {
    kind: GateType,
    inputs_indices: [usize; 2],
    output_index: usize,
}

struct Circuit {
    initial_wire_states: HashMap<String, WireState>,
    wires: Vec<Wire>,
    logic_gates: Vec<LogicGate>,
}

impl Circuit {
    fn parse_input(input: &str) -> Circuit {
        let mut lines = input.trim().lines();

        let mut circuit = Circuit {
            initial_wire_states: HashMap::new(),
            wires: vec![],
            logic_gates: vec![],
        };

        // Example: "x00: 1"
        let wire_state_regex = Regex::new("([^:]+): ([10])").unwrap();

        // Example "x00 AND y00 -> z00"
        let logic_gate_regex = Regex::new("([^ ]+) ([^ ]+) ([^ ]+) -> ([^ ]+)").unwrap();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let (_, [wire_name, initial_state]) =
                wire_state_regex.captures(line).unwrap().extract();

            let wire = Wire {
                name: wire_name.to_string(),
                state: None,
                dependency_indices: vec![],
            };

            circuit
                .initial_wire_states
                .insert(wire_name.to_string(), WireState::from_str(initial_state));
            circuit.wires.push(wire);
        }

        while let Some(line) = lines.next() {
            let (_, [wire_one, gate, wire_two, output]) =
                logic_gate_regex.captures(line).unwrap().extract();

            let gate = LogicGate {
                kind: GateType::from_str(gate),
                inputs_indices: [
                    circuit.get_or_insert_wire_index(wire_one),
                    circuit.get_or_insert_wire_index(wire_two),
                ],
                output_index: circuit.get_or_insert_wire_index(output),
            };

            circuit.wires[gate.inputs_indices[0]]
                .dependency_indices
                .push(circuit.logic_gates.len());
            circuit.wires[gate.inputs_indices[1]]
                .dependency_indices
                .push(circuit.logic_gates.len());

            circuit.logic_gates.push(gate);
        }

        circuit
    }

    fn get_or_insert_wire_index(&mut self, name: &str) -> usize {
        if let Some(index) = self.wires.iter().position(|wire| wire.name == name) {
            return index;
        }

        let new_index = self.wires.len();
        self.wires.push(Wire {
            name: name.to_string(),
            state: None,
            dependency_indices: vec![],
        });

        new_index
    }

    fn simulate(&mut self) {
        let mut update_queue = VecDeque::from_iter(self.initial_wire_states.iter());

        while let Some(update) = update_queue.pop_front() {}
    }
}

impl Solution for Day24 {
    fn part1(&self, input: &str) -> String {
        let mut circuit = Circuit::parse_input(input);

        circuit.simulate();
        circuit
            .wires
            .iter()
            .filter(|wire| wire.name.starts_with("z"))
            .sorted_by(|lhs, rhs| lhs.name.cmp(&rhs.name))
            .for_each(|wire| println!("{} -> {:?}", wire.name, wire.state));
        println!("{} wires", circuit.wires.len());
        println!("{} logic gates", circuit.logic_gates.len());

        todo!()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}
