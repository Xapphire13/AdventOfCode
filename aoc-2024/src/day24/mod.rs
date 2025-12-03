use std::{
    collections::VecDeque,
    ops::{BitAnd, BitOr, BitXor},
};

use itertools::Itertools;
use regex::Regex;

use shared::Solution;

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
            unknown => panic!("Unknown state: {unknown}"),
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

#[derive(PartialEq, Eq)]
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
            unknown => panic!("Unknown gate type: {unknown}"),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            GateType::And => "AND",
            GateType::Or => "OR",
            GateType::Xor => "XOR",
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
    initial_wire_states: Vec<(usize, WireState)>,
    wires: Vec<Wire>,
    logic_gates: Vec<LogicGate>,
}

impl Circuit {
    fn parse_input(input: &str) -> Circuit {
        let mut lines = input.trim().lines();

        let mut circuit = Circuit {
            initial_wire_states: vec![],
            wires: vec![],
            logic_gates: vec![],
        };

        // Example: "x00: 1"
        let wire_state_regex = Regex::new("([^:]+): ([10])").unwrap();

        // Example "x00 AND y00 -> z00"
        let logic_gate_regex = Regex::new("([^ ]+) ([^ ]+) ([^ ]+) -> ([^ ]+)").unwrap();

        for line in lines.by_ref() {
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
                .push((circuit.wires.len(), WireState::from_str(initial_state)));
            circuit.wires.push(wire);
        }

        for line in lines {
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

    fn get_wire_index(&self, name: &str) -> Option<usize> {
        self.wires.iter().position(|wire| wire.name == name)
    }

    fn get_or_insert_wire_index(&mut self, name: &str) -> usize {
        if let Some(index) = self.get_wire_index(name) {
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
            .map(|update| (update.0, update.1.clone()))
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

    fn print_wire(&self, wire_name: &str, depth: usize) {
        print!("{}", "     |".repeat(depth));

        if depth >= 6 {
            println!("...");
            return;
        }

        if ['x', 'y'].contains(&wire_name.chars().nth(0).unwrap()) {
            println!("- {wire_name}");
        } else {
            let wire_index = self.get_wire_index(wire_name).unwrap();
            let parent_gate = self
                .logic_gates
                .iter()
                .find(|gate| gate.output_index == wire_index)
                .unwrap();

            println!("{} ←[{}]", wire_name, parent_gate.kind.to_str());

            let input_1 = parent_gate.inputs_indices[0];
            let input_2 = parent_gate.inputs_indices[1];
            self.print_wire(&self.wires[input_1].name, depth + 1);
            self.print_wire(&self.wires[input_2].name, depth + 1);
        }
    }

    fn get_parent_gate(&self, wire_name: &str) -> &LogicGate {
        let wire_index = self.get_wire_index(wire_name).unwrap();

        self.logic_gates
            .iter()
            .find(|gate| gate.output_index == wire_index)
            .unwrap()
    }

    fn verify(&self, wire_name: &str) -> bool {
        if !wire_name.starts_with("z") {
            return false;
        }

        let bit_number = &wire_name[1..];
        let parent_gate = self.get_parent_gate(wire_name);

        if bit_number == "45" && parent_gate.kind == GateType::Or {
            return true;
        }

        if parent_gate.kind != GateType::Xor {
            println!("Parent not XOR");
            return false;
        }

        if bit_number == "00" {
            return true;
        }

        let input_1 = &self.wires[parent_gate.inputs_indices[0]];
        let input_2 = &self.wires[parent_gate.inputs_indices[1]];

        let xy_xor = [input_1, input_2].into_iter().find(|wire| {
            let parent_gate = self.get_parent_gate(&wire.name);
            parent_gate.kind == GateType::Xor
        });
        let carry = [&input_1, &input_2].into_iter().find(|wire| {
            let parent_gate = self.get_parent_gate(&wire.name);
            if bit_number == "01" {
                parent_gate.kind == GateType::And
            } else {
                parent_gate.kind == GateType::Or
            }
        });

        if let Some(xy_xor) = xy_xor {
            let parent_gate = self.get_parent_gate(&xy_xor.name);
            let inputs = [
                &self.wires[parent_gate.inputs_indices[0]].name,
                &self.wires[parent_gate.inputs_indices[1]].name,
            ];

            if !inputs
                .iter()
                .any(|&wire_name| wire_name == &format!("x{bit_number}"))
                || !inputs
                    .iter()
                    .any(|&wire_name| wire_name == &format!("y{bit_number}"))
            {
                println!("Wrong XY values");
                return false;
            }
        } else {
            println!("No input XOR");
            return false;
        }

        if carry.is_none() {
            println!("No carry bit");
            return false;
        }

        true
    }
}

fn get_result(circuit: &Circuit, prefix: &str) -> u64 {
    circuit
        .wires
        .iter()
        .filter(|wire| wire.name.starts_with(prefix))
        .sorted_by(|lhs, rhs| lhs.name.cmp(&rhs.name))
        .flat_map(|wire| wire.state.clone().map(|state| state.to_int()))
        .enumerate()
        .fold(0, |acc, (i, curr)| acc + (curr << i))
}

impl Solution for Day24 {
    fn part1(&self, input: &str) -> String {
        let mut circuit = Circuit::parse_input(input);

        circuit.simulate();
        let result = get_result(&circuit, "z");

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let circuit = Circuit::parse_input(input);

        for bit in 0..46 {
            let wire_name = format!("z{bit:02}");
            print!("Verifying {wire_name}... ");
            let result = circuit.verify(&wire_name);
            println!("{}", if result { "PASS!" } else { "FAIL!" });

            if !result {
                circuit.print_wire(&wire_name, 0);
                println!();
            }
        }

        // Adder value = x XOR y XOR pcarry
        // Adder carry = (x AND y) OR ((x XOR y) AND pcarry)
        //
        // Example:
        // z <-[XOR]
        //     |...-[XOR]
        //     |    |...-x
        //     |    |...-y
        //     |...-[OR]
        //          |...-[AND]
        //          |    |...-x'
        //          |    |...-y'
        //          |...-[AND]
        //               |...-pcarry (x'' y'')
        //               |...-[XOR]
        //                    |...-x'
        //                    |...-y'

        // Found by inspecting output of above and comparing with wiring diagram
        let swapped_gates = ["krj", "bpt", "ngr", "z11", "fkp", "z06", "mfm", "z31"];

        swapped_gates.iter().sorted().join(",")
    }
}
