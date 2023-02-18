// --- Day 7: Some Assembly Required ---
// This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates! Unfortunately, little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

// Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

// The included instructions booklet describes how to connect the parts together: x AND y -> z means to connect wires x and y to an AND gate, and then connect its output to wire z.

// For example:

// 123 -> x means that the signal 123 is provided to wire x.
// x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
// p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
// NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.
// Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If, for some reason, you'd like to emulate the circuit instead, almost all programming languages (for example, C, JavaScript, or Python) provide operators for these gates.

// For example, here is a simple circuit:

// 123 -> x
// 456 -> y
// x AND y -> d
// x OR y -> e
// x LSHIFT 2 -> f
// y RSHIFT 2 -> g
// NOT x -> h
// NOT y -> i
// After it is run, these are the signals on the wires:

// d: 72
// e: 507
// f: 492
// g: 114
// h: 65412
// i: 65079
// x: 123
// y: 456
// In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?

use std::collections::HashMap;

pub fn get_circuit_input(input: String, wire: &str) -> usize {
    let mut map: HashMap<&str, usize> = HashMap::new();
    while map.contains_key(wire) == false {
        for line in input.lines().clone() {
            let v = line.split_ascii_whitespace().collect::<Vec<&str>>();
            match v.len() {
                // a -> b
                3 => direct_set_signal(&v, &mut map),
                // NOT a -> b
                4 => not_operation(&v, &mut map),
                // a AND b -> c
                5 => two_arg_operation(&v, &mut map),
                _ => {}
            }
        }
    }
    println!("{:#?}", map);
    let b = map[wire];
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("b", b);
    while map.contains_key(wire) == false {
        for line in input.lines().clone() {
            let v = line.split_ascii_whitespace().collect::<Vec<&str>>();
            match v.len() {
                // a -> b
                3 => direct_set_signal(&v, &mut map),
                // NOT a -> b
                4 => not_operation(&v, &mut map),
                // a AND b -> c
                5 => two_arg_operation(&v, &mut map),
                _ => {}
            }
        }
    }

    map[wire]
}

fn direct_set_signal<'a>(v: &Vec<&'a str>,map: &mut HashMap<&'a str, usize>){
    if v[2] == "b" && map.contains_key("b") == true {
        return;
    }
    match v[0].parse::<usize>() {
        Ok(num) => {
            map.insert(v[2], num);
        }
        Err(_) => {
            if map.contains_key(v[0]) == true {
                map.insert(v[2], map[v[0]]);
            }
        }
    }
}

fn not_operation<'a>(v: &Vec<&'a str>, map: &mut HashMap<&'a str, usize>) {
    if v[3] == "b" && map.contains_key("b") == true {
        return;
    }
    
    match v[1].parse::<usize>() {
        Ok(num) => {
            map.insert(v[3], !num & 0xffff);
        }
        Err(_) => {
            if map.contains_key(v[1]) == true {
                map.insert(v[3], !map[v[1]] & 0xffff);
            }
        }
    }
}

fn two_arg_operation<'a>(v: &Vec<&'a str>, map: &mut HashMap<&'a str, usize>) {
    if v[4] == "b" && map.contains_key("b") == true{
        return;
    }
    let a = match v[0].parse::<usize>() {
        Ok(num) => Some(num),
        Err(_) => match map.contains_key(v[0]) {
            true => Some(map[v[0]]),
            false => None,
        },
    };

    let b = match v[2].parse::<usize>() {
        Ok(num) => Some(num),
        Err(_) => match map.contains_key(v[2]) {
            true => Some(map[v[2]]),
            false => None,
        },
    };
    let  _= match (a, b) {
        (Some(x), Some(y)) => {
            match v[1] {
                "AND" => map.insert(v[4], x & y),
                "OR" => map.insert(v[4], x | y),
                "LSHIFT" => map.insert(v[4], x << y),
                "RSHIFT" => map.insert(v[4], x >> y),
                _ => None,
            }
        }
        _ => None,
    };
}
