use itertools::Itertools;

use crate::solution::{load, Solution};
use std::{io::BufRead, collections::VecDeque};

pub struct Day11;

const YEAR: u32 = 2022;
const DAY: u32 = 11;

impl Solution for Day11 {
    fn name(&self) -> String {
        "Monkey in the Middle".to_string()
    }

    /// Part A
    /// Monkeys are playing with your things. They toss your things around based off your woes.
    /// Determine how many items each monkey has inspected.
    /// Then determine the monkey business by multiplying the top 2 monkeys # of inspected items after 20 rounds.
    fn part_a(&self) {
        let br = load(YEAR, DAY);
        let lines = br.lines();
        let mut monkeys: Vec<Monkey> = vec![];
        let monkey_ref = &mut monkeys;

        get_monkeys(lines, monkey_ref);

        // for (i, m) in monkey_ref.iter().enumerate() {
        //     println!("Monkey {i} has Items: {:?}", m.items);
        //     println!("Monkey {i} has operand: {:?}", m.operand);
        // }

        run_monkey_game(monkey_ref, 20, |x| x / 3);

        let mut num_inspected: Vec<usize> = vec![];
        for (_i, m) in monkeys.iter().enumerate() {
            // println!("Monkey {i} has inspected: {:?}", m.num_inspected);
            num_inspected.push(m.num_inspected);
        }
        num_inspected.sort();
        let worry = num_inspected.pop().unwrap() * num_inspected.pop().unwrap();
        println!("Top two monkeys items inspected multiplied by eachother: {}", worry);

    }

    /// Part B
    /// 10,000 rounds and this time the worry divisor is not 3
    /// So comparing to part 1 we have 20/3 vs 10000/1
    /// We are afraid of buffer overflow so a common solution is to use modulus in some sort of way
    /// Branching from the idea of least common multiple, we can get each monkey's divisor
    /// and multiply them all together, with this we can just use modulo of this product for each monkey's operation
    /// 
    /// mod = Product of All Monkeys' divisor
    /// For each operand in monkey operation -> Take modulo
    /// At the end we multiply the top two monkeys
    /// Todo - review discrete math and modulo arithmetic
    fn part_b(&self) {
        let br = load(YEAR, DAY);
        let lines = br.lines();
        let mut monkeys: Vec<Monkey> = vec![];
        let monkey_ref = &mut monkeys;
        get_monkeys(lines, monkey_ref);
        let mut modu = 1;
        monkey_ref.iter().for_each(|m| modu *= m.monkey_test.operand);
        run_monkey_game(monkey_ref, 10000, |x| x % modu);
        let mut num_inspected: Vec<usize> = vec![];
        for (_i, m) in monkeys.iter().enumerate() {
            // println!("Monkey {i} has inspected: {:?}", m.num_inspected);
            num_inspected.push(m.num_inspected);
        }
        num_inspected.sort();
        let worry = num_inspected.pop().unwrap() * num_inspected.pop().unwrap();
        println!("Top two monkeys items inspected multiplied by eachother: {}", worry);
    }
}

fn run_monkey_game(monkey_ref: &mut Vec<Monkey>, rounds: usize, worry_level_divisor: impl Fn(usize) -> usize) {
    for _x in 0..rounds {
        // println!("Round: {x}");
        // for (i, m) in monkey_ref.iter().enumerate() {
        //     println!("\tMonkey {i} has Items: {:?}", m.items);
        // }
        for i in 0..monkey_ref.len() {
            // println!("\t\tMonkey {i} round actions:");
            let mut item_len = monkey_ref[i].items.len();
            while item_len > 0 {
                let m = &mut monkey_ref[i];
                m.num_inspected += 1;
                let item_thrown = m.items.pop_front().unwrap();
                item_len -= 1;
                let operand = match m.operand.as_str() {
                    "old" => {
                        item_thrown
                    },
                    _ => { m.operand.parse::<usize>().unwrap() }
                };
                // We can the product of all monkey's divisor check here to lower the operands so we dont get an overflow
                let worry = worry_level_divisor((m.operation)(item_thrown, operand));
                let test_op = m.monkey_test.operand;
                let test = (m.monkey_test.test)(worry, test_op);
                let target = if test { m.monkey_test.toss_targets.0 } else { m.monkey_test.toss_targets.1 };
                // println!("\t\tItem worry level:{worry}, sent to monkey: {target}");
                let m = &mut monkey_ref[target];
                m.items.push_back(worry);
            }
        }
    }
}

fn get_monkeys(lines: std::io::Lines<std::io::BufReader<std::fs::File>>, monkeys: &mut Vec<Monkey>) {
    // chunk lines by 7 -- we have monkey, items, operation, test, + if + else + new line to separate
    for chunk in lines.chunks(7).into_iter() {            
        let mut items: VecDeque<usize> = VecDeque::new();
        let mut operation: Box<dyn Fn(usize, usize) -> usize> = Box::new(|_x, _y| 0);
        let mut operand: String = String::from("");
        let mut test: Box<dyn Fn(usize, usize) -> bool> = Box::new(|_x, _y| false);
        let mut test_operand: usize = 0;
        let mut throw_target_1 = 0;
        let mut throw_target_2 = 0;

        for line in chunk {
            if let Ok(l) = line {
                let l = l.trim().to_string();
                if l.starts_with("Starting items: ") {
                    items = l.trim_start_matches("Starting items: ")
                        .split(", ")
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect();
                } else if l.starts_with("Operation: ") {
                    let op_operand: Vec<&str> = l.trim_start_matches("Operation: new = old ")
                        .split(" ")
                        .collect();
                    operand = op_operand[1].to_string().trim().to_string();
                    operation = match op_operand[0] {
                        "*" => {
                            Box::new(|old, operand| old * operand)
                        },
                        "+" => {
                            Box::new(|old, operand| old + operand)
                        },
                        _ => { 
                            // should never be here, only * and + are in the input
                            Box::new(|old, _operand| old)
                        }
                    }
                } else if l.starts_with("Test: ") {
                    test_operand = l
                        .trim_start_matches("Test: divisible by ")
                        .parse::<usize>()
                        .unwrap();
                    test = Box::new(|old, op| old % op == 0);
                } else if l.starts_with("If true") {
                    throw_target_1 = l.trim_start_matches("If true: throw to monkey ")
                        .parse::<usize>().unwrap();
                } else if l.starts_with("If false") {
                    throw_target_2 = l.trim_start_matches("If false: throw to monkey ")
                        .parse::<usize>().unwrap();
                } else {
                    // either new line or monkey
                    continue;
                }
            }
        }

        // create new monkey and push to monkeys vec
        let monkey_test: MonkeyTest = MonkeyTest::new(test, (throw_target_1, throw_target_2), test_operand);
        let monkey: Monkey = Monkey::new(items, operation, operand, monkey_test);
        monkeys.push(monkey);
    }
}

struct MonkeyTest {
    test: Box<dyn Fn(usize, usize) -> bool>,
    toss_targets: (usize, usize),
    operand: usize,
}

impl MonkeyTest {
    fn new(test: Box<dyn Fn(usize, usize) -> bool>, targets: (usize, usize), operand: usize) -> MonkeyTest {
        MonkeyTest {
            test: test,
            toss_targets: targets,
            operand: operand,
        }
    }
}

struct Monkey {
    num_inspected: usize,

    items: VecDeque<usize>,

    // Operation can be a closure -> multiply, add, divide, etc... all possible..
    // https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust
    // Using a box closure stores the closure onto the heap so we dont have to worry about lifetimes
    /// Closure for monkey struct that takes in worry level and does an operation to spit out the new level
    operation: Box<dyn Fn(usize, usize) -> usize>,
    operand: String,

    monkey_test: MonkeyTest,
}

impl Monkey {
    fn new(
        items: VecDeque<usize>, 
        operation: Box<dyn Fn(usize, usize) -> usize>, 
        operand: String,
        monkey_test: MonkeyTest,
    ) -> Monkey {
        Monkey {
            num_inspected: 0,
            items: items,
            operation: operation,
            operand: operand,
            monkey_test: monkey_test,
        }
    }
}