use crate::solution::{load, Solution};
use std::io::BufRead;

pub struct Day10;

const YEAR: u32 = 2022;
const DAY: u32 = 10;

impl Solution for Day10 {
    fn name(&self) -> String {
        "Cathode-Ray Tube".to_string()
    }

    /// Part A
    /// During the 20th, 60th, 100th, 140th, 180th, and 220th cycle, what is the sum of the 6 signal strengths?
    /// Signal strength = cycle * register value.
    /// noop takes 1 cycle
    /// addx takes 2 cycles
    /// register x starts with a value of 1
    fn part_a(&self) {
        let br = load(YEAR, DAY);

        let debug_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
        // let debug_cycles: Vec<usize> = vec![0,1,2,3,4,5,6,7];
        let mut processor: Processor = Processor::new_from_debug_cycles(debug_cycles);

        for line in br.lines() {
            if let Ok(l) = line {
                // println!("{l}");
                let i = Instruction::new(l);
                processor.process_instr(i);
            }
        }

        println!("Signal strength: {}", processor.signal_strength);
    }

    /// Part B
    /// X register controls the horizontal position of the sprite (which is 3 pixels wide)
    /// Controls it via the middle of the sprite.
    /// CRT is 40 wide and 6 high. Which 1 starts at the top left.
    /// CRT draws a single pixel during each cycle
    fn part_b(&self) {
        let br = load(YEAR, DAY);

        let mut processor: Processor = Processor::new();

        for line in br.lines() {
            if let Ok(l) = line {
                let i = Instruction::new(l);
                processor.process_instr(i);
            }
        }
 
        println!("CRT: \n{}", processor.crt.disp);
        for (i,c) in processor.crt.disp.chars().enumerate() {
            if i % 40 == 0 {
                print!("\n");
            }
            print!("{c}");
        }
    }
}


#[derive(Debug)]
enum InstructionType {
    ADD,
    NOOP,
}

#[derive(Debug)]
struct Instruction {
    instr_type: InstructionType,
    instr: Vec<String>,
    cycles: u8,
}

impl Instruction {
    fn new(inst: String) -> Instruction {
        let i_p: Vec<&str> = inst.split(" ").collect();
        let instr = i_p[0];
        let instr_type: InstructionType = match instr {
            "noop" => InstructionType::NOOP,
            _ => InstructionType::ADD, // add and uknown are here
        };
        let cycles = match instr_type {
            InstructionType::NOOP => 1,
            InstructionType::ADD => 2,
        };
        Instruction {
            instr_type: instr_type,
            instr: i_p.iter().map(|&s|s.into()).collect(),
            cycles: cycles,
        }
    }
}

struct CRT {
    disp: String,
    width: usize,
}

impl CRT {
    fn new(width: usize) -> CRT {
        CRT { disp: String::from(""), width, }
    }

    fn draw(&mut self, x_pos: usize) {
        let len = self.disp.len();
        // within range = x_pos +- 1
        if x_pos.saturating_sub(1) <= len % self.width && len % self.width <= x_pos.saturating_add(1) {
            self.disp.insert(len, '#');
        } else {
            self.disp.insert(len, '.');
        }
    }
}

struct Processor {
    cycle: usize,
    registers: Vec<i32>,
    debug_cycles: Vec<usize>,
    signal_reg_index: usize,
    signal_strength: i32,
    crt: CRT,
}

impl Processor {
    fn new_from_debug_cycles(debug_cycles: Vec<usize>) -> Processor {
        Processor {
            cycle: 0,
            registers: vec![1; 26], // 26 characters - 26 registers (if we end up needing to add more registers?)
            debug_cycles: debug_cycles,
            signal_strength: 0,
            signal_reg_index: 'x' as usize - 'a' as usize,
            crt: CRT::new(40),
        }
    }

    fn new() -> Processor {
        Processor {
            cycle: 0,
            registers: vec![1; 26], // 26 characters - 26 registers (if we end up needing to add more registers?)
            debug_cycles: vec![],
            signal_strength: 0,
            signal_reg_index: 'x' as usize - 'a' as usize,
            crt: CRT::new(40),
        }
    }

    fn process_instr(&mut self, inst: Instruction) {
        let x_pos = self.registers[self.signal_reg_index];
        for _i in 0..inst.cycles {
            self.cycle += 1;
            self.crt.draw(x_pos as usize);
            let debug = self.debug_cycles.binary_search(&(self.cycle as usize));
            match debug {
                Ok(_d) => {
                    println!("  Register values at cycle {}.\n\t{:?}", self.cycle, self.registers);
                    // could be lossful conversion... what's an alternative
                    let cycle: i32 = self.cycle.try_into().unwrap();
                    self.signal_strength += self.registers[self.signal_reg_index] * cycle;
                },
                _ => {},
            }
        }

        match inst.instr_type {
            InstructionType::ADD => {
                let reg_i = self.get_register_index(&inst);
                let add_amt = inst.instr[1].parse::<i32>().unwrap();
                self.registers[reg_i] += add_amt;
            },
            _ => {/* do nothing */}
        }
        // println!("  Cycle now at: {} with reg values: {:?}", self.cycle, self.registers);
    }

    fn get_register_index(&self, instr: &Instruction) -> usize {
        let register = instr.instr[0].chars().next_back().unwrap();
        register as usize - 'a' as usize
    }
}