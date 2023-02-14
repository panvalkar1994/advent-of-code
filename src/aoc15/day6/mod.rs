// --- Day 6: Probably a Fire Hazard ---
// Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

// Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

// Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

// To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

// For example:

// turn on 0,0 through 999,999 would turn on (or leave on) every light.
// toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
// turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.
// After following the instructions, how many lights are lit?

pub fn get_all_lite_lights(input:String)->usize{
    let mut state = [[false;1000];1000];
    
    for line in input.lines() {
        let instruction:Instruction = parse_positions(line);
        update_lights_by_instruction(instruction, &mut state);
    }
    get_all_on_counts(&state)
}

#[derive(Debug)]
struct Point {
    x:usize,
    y:usize,
}

#[derive(Debug)]
enum Operation {
    On,
    Off,
    Toggle
}

#[derive(Debug)]
struct Instruction{
    start:Point,
    end:Point,
    op:Operation,
}

fn parse_positions(line:&str)->Instruction {
    let mut v = line.split_ascii_whitespace();
    let mut op:Operation = Operation::Off;
    let mut start = Point{ x:0, y:0 };
    let mut end = Point{ x:0, y:0 };
    match v.next() {
        Some("toggle")=>{
            op = Operation::Toggle;
        },
        Some("turn") => {
            match v.next() {
                Some("on") =>{
                    op = Operation::On;
                },
                Some("off") =>{
                    op = Operation::Off;
                },
                None=>{},
                Some(_)=>{}
            }
        },
        None=>{},
        Some(_)=>{}
    }

    if let Some(tup) = v.next() {
        
        start = get_point_from_str(tup);
    }
    let _ = v.next();
    if let Some(tup) = v.next() {

        end = get_point_from_str(tup);
    }

    Instruction { start, end, op }
}

fn get_point_from_str(s:&str)->Point {
    let v = s.split(',').collect::<Vec<&str>>();
    Point { x: v[0].parse().unwrap(), y: v[1].parse().unwrap() }
}

fn update_lights_by_instruction(step:Instruction, state:&mut [[bool;1000];1000]) {
    for i in step.start.x..=step.end.x{
        for j in step.start.y..=step.end.y{
            match step.op {
                Operation::Toggle => {
                    state[i][j] = !state[i][j];
                },
                Operation::Off => {
                    state[i][j] = false;
                },
                Operation::On => {
                    state[i][j] = true;
                }
            }
        }
    }
}

fn get_all_on_counts(state:&[[bool;1000];1000]) -> usize {
    let mut count = 0;
    for i in 0..1000{
        for j in 0..1000{
            if state[i][j] == true {
                count += 1;
            }
        }
    }
    count
}