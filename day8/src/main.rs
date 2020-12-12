
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Opcode {
    Nop,
    Acc,
    Jmp
}

#[derive(PartialEq, Debug)]
struct Instruction {
    opcode: Opcode,
    argument: isize
}

impl Instruction {
    pub fn parse(s: &str) -> Self {
        let mut tokens = s.trim().split(' ');
        let opcode_str = tokens.next().unwrap();
        let opcode = match opcode_str {
            "nop" => Opcode::Nop,
            "acc" => Opcode::Acc,
            "jmp" => Opcode::Jmp,
            _     => panic!("Invalid opcode: {}", opcode_str)
        };
        let argument_str = tokens.next().unwrap();
        let argument = argument_str.parse().unwrap();

        Self{
            opcode,
            argument
        }
    }
}

fn assemble(source: &str) -> Vec<Instruction> {
    let line_count = source.split('\n').count();
    let mut code = Vec::with_capacity(line_count);
    for line in source.split('\n').filter(|l| !l.is_empty()) {
        code.push(Instruction::parse(line));
    }
    code
}

/// Runs the code and returns the final accumulator value. The second tuple element indicates
/// whether the code terminated normally (true) or was stopped due to an infinite loop (false).
fn run(code: &Vec<Instruction>) -> (isize, bool, Vec<bool>) {
    let mut executed = Vec::new();
    executed.resize(code.len(), false);

    let mut program_counter : isize = 0;
    let mut accumulator : isize = 0;
    while program_counter >= 0 && (program_counter as usize) < code.len() {
        let pc = program_counter as usize;
        if executed[pc] {
            return (accumulator, false, executed);
        }
        let fetched = &code[pc];
        let mut next_pc = program_counter + 1;
        match fetched.opcode {
            Opcode::Nop => { },
            Opcode::Acc => accumulator += fetched.argument,
            Opcode::Jmp => next_pc = program_counter + fetched.argument
        }
        program_counter = next_pc;
        executed[pc] = true;
    }

    (accumulator, true, executed)
}

enum TraceResult {
    Fixed(usize),
    Terminated
}

fn trace(code: &Vec<Instruction>, was_executed: &Vec<bool>, jump_map: &HashMap<usize, Vec<usize>>, trace_start: usize) -> TraceResult {
    println!("---- Begin trace at {} ----", trace_start);

    // now we walk backwards from the instruction after the last
    let mut pc = trace_start;
    while pc > 0 {

        println!("Checking {}", pc);

        // if there is a NOP that would lead here _and_ was executed, we need to turn it into a JMP
        if let Some(source_list) = jump_map.get(&pc) {
            println!("  This is a potential target for a jump (correct offsets exist)");
            for source in source_list {
                if code[*source].opcode == Opcode::Nop && was_executed[*source] {
                    println!("  Found a matching NOP that was executed at {}", *source);
                    return TraceResult::Fixed(*source);
                }
            }
            println!("    No executed NOPs can lead here");
        }else{
            println!("  There are not offsets pointing here");
        }

        let mut next_pc = pc - 1;

        // if the instruction before this one is a JMP that was was executed, it is the one we
        //  need to change to a NOP. if it is a JMP that was not executed, we need to find a JMP
        //  that leads to the current instruction (and was not executed, but that is implied) and
        //  continue our search from there (that has to be a unique location, however, because
        //  otherwise things get really complicated).
        if code[pc-1].opcode == Opcode::Jmp {
            println!("  The previous instruction was a JMP");
            if was_executed[pc-1] {
                println!("  This JMP was executed. Changing it will fix the code");
                return TraceResult::Fixed(pc-1);
            }else{
                println!("  This JMP was not executed. Need to find out how we can get here");
                if let Some(source_list) = jump_map.get(&pc) {
                    let mut potential_single_source = None;
                    let mut potential_source_count = 0;
                    for source in source_list {
                        if code[*source].opcode == Opcode::Jmp {
                            println!("    A JMP at {} can lead here", *source);
                            potential_single_source = Some(*source);
                            potential_source_count += 1;
                        }
                    }

                    if potential_source_count == 0 {
                        println!("    No potential source can lead to this point in the code. This path terminates");
                        return TraceResult::Terminated;
                    }else if potential_source_count == 1 {
                        println!("  Only a single source can lead here. Going there");
                        next_pc = potential_single_source.unwrap();
                    }else{
                        println!("  Multiple JMPs can lead here. Need to branch");
                        for source in source_list {
                            if code[*source].opcode == Opcode::Jmp {
                                if let TraceResult::Fixed(loc) = trace(code, was_executed, jump_map, *source) {
                                    return TraceResult::Fixed(loc);
                                }
                            }
                        }
                        println!("    None of the branches from {} fixed the code. This path terminates", pc);
                        return TraceResult::Terminated;
                    }
                }else{
                    println!("    No potential source can lead to this point in the code. This path terminates");
                    return TraceResult::Terminated;
                }
            }
        }

        pc = next_pc;
    }

    panic!("Trace moved out of valid address range");
}

/// Tries to find a single instruction to change so the code terminates. This needs the
//  executed-flags from a previous run-step. Returns the index of the instruction that needs to be
//  changed.
fn find_code_fix_location(code: &Vec<Instruction>, was_executed: &Vec<bool>) -> usize {

    // for every address, we need list of NOP or JMP addressed that could lead there (turn all GOTOs into COMEFROMs)
    let mut jump_map : HashMap<usize, Vec<usize>> = HashMap::with_capacity(code.len());
    for (address, instruction) in code.iter().enumerate().filter(|i| i.1.opcode != Opcode::Acc) {
        let target = ((address as isize) + instruction.argument) as usize;
        if let Some(source_list) = jump_map.get_mut(&target) {
            source_list.push(address);
        }else{
            let mut source_list = Vec::new();
            source_list.push(address);
            jump_map.insert(target, source_list);
        }
    }

    if let TraceResult::Fixed(loc) = trace(code, was_executed, &jump_map, code.len()) {
        loc
    }else{
        panic!("Failed to fix the code");
    }
}

fn flip_nop_jmp(i: &mut Instruction) {
    match i.opcode {
        Opcode::Nop => i.opcode = Opcode::Jmp,
        Opcode::Jmp => i.opcode = Opcode::Nop,
        Opcode::Acc => panic!("Not a NOP or JMP")
    }
}

fn main() {
    let input = std::fs::read_to_string("day8/input.txt").unwrap();
    let mut code = assemble(&input);
    let run_result = run(&code);
    println!("Final accumulator value before fixing: {}", run_result.0);

    let fix_location = find_code_fix_location(&mut code, &run_result.2);
    println!("Fixed code by changing instruction at {}", fix_location);

    flip_nop_jmp(&mut code[fix_location]);

    let fixed_run_result = run(&code);
    println!("Fixed code terminated: {}", fixed_run_result.1);
    println!("Final accumulator value: {}", fixed_run_result.0);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_ASM : &str = "nop +0
                             acc +1
                             jmp +4
                             acc +3
                             jmp -3
                             acc -99
                             acc +1
                             jmp -4
                             acc +6";

    #[test]
    fn assembly() {
        let assembled = assemble(TEST_ASM);
        assert_eq!(assembled.len(), 9);
        assert_eq!(assembled[0].opcode, Opcode::Nop);
        assert_eq!(assembled[0].argument, 0);
        assert_eq!(assembled[2].opcode, Opcode::Jmp);
        assert_eq!(assembled[2].argument, 4);
        assert_eq!(assembled[5].opcode, Opcode::Acc);
        assert_eq!(assembled[5].argument, -99);
    }

    #[test]
    fn running() {
        let assembled = assemble(TEST_ASM);
        let run_result = run(&assembled);
        assert_eq!(run_result.0, 5);
        assert_eq!(run_result.1, false);
    }

    #[test]
    fn fixing() {
        let mut assembled = assemble(TEST_ASM);
        let run_result = run(&assembled);
        let fix_location = find_code_fix_location(&mut assembled, &run_result.2);
        assert_eq!(fix_location, 7);

        flip_nop_jmp(&mut assembled[fix_location]);

        let fixed_run_result = run(&assembled);
        assert_eq!(fixed_run_result.0, 8);
        assert_eq!(fixed_run_result.1, true);
    }

}
