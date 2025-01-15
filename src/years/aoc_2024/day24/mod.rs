use std::{
    collections::{HashMap, VecDeque},
    ops::{BitAnd, BitOr, BitXor},
};

use itertools::Itertools;
use regex::Regex;

use crate::aoc_solution::Solution;

pub struct Day24;

#[derive(Debug, Clone, PartialEq, Eq)]
enum WireState {
    On,
    Off,
}

impl BitAnd for WireState {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self == WireState::On && rhs == WireState::On {
            WireState::On
        } else {
            WireState::Off
        }
    }
}

impl BitOr for WireState {
    type Output = WireState;

    fn bitor(self, rhs: Self) -> Self::Output {
        if self == WireState::On || rhs == WireState::On {
            WireState::On
        } else {
            WireState::Off
        }
    }
}

impl BitXor for WireState {
    type Output = WireState;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self != rhs && (self | rhs) == WireState::On {
            WireState::On
        } else {
            WireState::Off
        }
    }
}

impl WireState {
    fn from_str(value: &str) -> WireState {
        match value {
            "1" => WireState::On,
            "0" => WireState::Off,
            unknown => panic!("Unknown state: {}", unknown),
        }
    }

    fn to_int(&self) -> u64 {
        match self {
            WireState::On => 1,
            WireState::Off => 0,
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

impl LogicGate {
    fn compute(&self, input_1: &WireState, input_2: &WireState) -> WireState {
        match self.kind {
            GateType::And => input_1.clone() & input_2.clone(),
            GateType::Or => input_1.clone() | input_2.clone(),
            GateType::Xor => input_1.clone() ^ input_2.clone(),
        }
    }
}

struct Circuit {
    initial_wire_states: HashMap<usize, WireState>,
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
                .insert(circuit.wires.len(), WireState::from_str(initial_state));
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
        let mut update_queue: VecDeque<(usize, WireState)> = self
            .initial_wire_states
            .iter()
            .map(|update| (update.0.clone(), update.1.clone()))
            .collect();

        while let Some((wire_index, state)) = update_queue.pop_front() {
            let dependency_indices = {
                let wire = &mut self.wires[wire_index];
                wire.state = Some(state);
                wire.dependency_indices.iter().cloned().collect_vec()
            };

            for gate_index in dependency_indices {
                let gate = &self.logic_gates[gate_index];
                let wire_1 = &self.wires[gate.inputs_indices[0]];
                let wire_2 = &self.wires[gate.inputs_indices[1]];

                if let (Some(input_1), Some(input_2)) = (&wire_1.state, &wire_2.state) {
                    let result = gate.compute(input_1, input_2);
                    update_queue.push_back((gate.output_index, result));
                }
            }
        }
    }
}

impl Solution for Day24 {
    fn part1(&self, input: &str) -> String {
        let mut circuit = Circuit::parse_input(input);

        circuit.simulate();
        let result = circuit
            .wires
            .iter()
            .filter(|wire| wire.name.starts_with("z"))
            .sorted_by(|lhs, rhs| lhs.name.cmp(&rhs.name))
            .flat_map(|wire| wire.state.clone().and_then(|state| Some(state.to_int())))
            .enumerate()
            .fold(0, |acc, (i, curr)| acc + (curr << i));

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}
