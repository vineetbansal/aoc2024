#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_u8(index: u8) -> Option<Self> {
        use Instruction::*;
        match index {
            0 => Some(Adv),
            1 => Some(Bxl),
            2 => Some(Bst),
            3 => Some(Jnz),
            4 => Some(Bxc),
            5 => Some(Out),
            6 => Some(Bdv),
            7 => Some(Cdv),
            _ => None,
        }
    }
}

struct Computer {
    registers: [u64; 3], // A, B, C
    ip: usize,
    instr: Option<Instruction>,
    operand: Option<u64>,
    output: Vec<u64>,
    tape: Vec<u64>,
}

impl Computer {
    fn new(tape: Vec<u64>, registers: Option<[u64; 3]>) -> Self {
        Self {
            registers: registers.unwrap_or([0, 0, 0]),
            ip: 0,
            instr: None,
            operand: None,
            output: vec![],
            tape,
        }
    }

    fn from_string(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let registers: [u64; 3] = [
            lines[0].split(": ").nth(1).unwrap().parse().unwrap(),
            lines[1].split(": ").nth(1).unwrap().parse().unwrap(),
            lines[2].split(": ").nth(1).unwrap().parse().unwrap(),
        ];
        let tape: Vec<u64> = lines[4].split(": ").nth(1).unwrap().split(",").map(|s| s.parse().unwrap()).collect();
        Self {
            registers,
            ip: 0,
            instr: None,
            operand: None,
            output: vec![],
            tape,
        }
    }

    fn execute(&mut self) {
        while self.ip < self.tape.len() {
            self.read();
            if let Some(instr) = self.instr {
                match instr {
                    Instruction::Adv => self.adv(),
                    Instruction::Bxl => self.bxl(),
                    Instruction::Bst => self.bst(),
                    Instruction::Jnz => self.jnz(),
                    Instruction::Bxc => self.bxc(),
                    Instruction::Out => self.out(),
                    Instruction::Bdv => self.bdv(),
                    Instruction::Cdv => self.cdv(),
                }
            }
        }
    }

    fn get_register(&self, which: char) -> u64 {
        match which {
            'A' => self.registers[0],
            'B' => self.registers[1],
            'C' => self.registers[2],
            _ => panic!("Invalid register!"),
        }
    }

    fn set_register(&mut self, which: char, value: u64) {
        match which {
            'A' => self.registers[0] = value,
            'B' => self.registers[1] = value,
            'C' => self.registers[2] = value,
            _ => panic!("Invalid register!"),
        }
    }

    fn get_literal(&self) -> u64 {
        self.operand.unwrap()
    }

    fn get_combo(&self) -> u64 {
        let value = self.operand.unwrap();
        match value {
            0..=3 => value.into(),
            4 => self.get_register('A'),
            5 => self.get_register('B'),
            6 => self.get_register('C'),
            _ => panic!("Invalid combo value!"),
        }
    }

    fn get_output(&self) -> String {
        self.output.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(",")
    }

    fn read(&mut self) {
        self.instr = Instruction::from_u8(self.tape[self.ip] as u8);
        self.operand = Some(self.tape[self.ip + 1]);
        self.ip += 2;
    }

    fn adv(&mut self) {
        let value = self.get_register('A') / (2u64.pow(self.get_combo() as u32));
        self.set_register('A', value);
    }

    fn bxl(&mut self) {
        let value = self.get_register('B') ^ self.get_literal();
        self.set_register('B', value);
    }

    fn bst(&mut self) {
        let value = self.get_combo() % 8;
        self.set_register('B', value);
    }

    fn jnz(&mut self) {
        if self.get_register('A') != 0 {
            self.ip = self.get_literal() as usize;
        }
    }

    fn bxc(&mut self) {
        let value = self.get_register('B') ^ self.get_register('C');
        self.set_register('B', value);
    }

    fn out(&mut self) {
        let value = self.get_combo() % 8;
        self.output.push(value);
    }

    fn bdv(&mut self) {
        let value = self.get_register('A') / (2u64.pow(self.get_combo() as u32));
        self.set_register('B', value);
    }

    fn cdv(&mut self) {
        let value = self.get_register('A') / (2u64.pow(self.get_combo() as u32));
        self.set_register('C', value);
    }
}

fn solution_a(input: &str) -> String {
    let mut computer = Computer::from_string(input);
    computer.execute();
    computer.get_output()
}

fn solution_b(input: &str) -> String {
    /*
    Run through program
    2,4,1,1,7,5, 1,5,4,0,5,5,0,3,3,0
    2,4,                                   |A|%8 -> B
        1,1,                               |B|^1 -> B
            7,5,                           |A|/2**B -> C
                1,5,                       |B|^5 -> B
                    4,x,                   |B|^|C| -> B
                        5,5,               output B
                            0,3,           |A|/2**3 -> A
                                3,0        if A is nonzero, jump to start

    Observations:
    We only ever output B, and only once in the above program.
    After dividing A by 8, we jump to start, which we need to do |length_of_output| times
    Nothing modifies A except the above division.
    The value of A determines B, which is the only thing we output.
    So the value of A solely determines the output digit at any step!
    We can try to run through all A values and print the output in reverse digit-by-digit,
    knowing that A needs to jump by (at least) 8 times its previous value to get the next digit.
    Once we have the desired output, we have the A value that should have been the initial A.
     */

    let tape: Vec<u64> = input.lines().nth(4).unwrap().split(": ").nth(1).unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    let mut a_values: Vec<u64> = vec![];

    for _ in 0..tape.len() {
        let mut a = if a_values.len() > 0 { 8*a_values[a_values.len()-1] } else { 0 };
        loop {
            let mut computer = Computer::new(tape.clone(), Some([a, 0, 0]));
            computer.execute();
            let output = computer.output;
            if output == tape[tape.len()-output.len()..] {
                a_values.push(a);
                break;
            }
            a += 1;
        }
    }

    a_values[a_values.len()-1].to_string()
}

fn main() {
    aoc2024::run("17", solution_a, solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let input: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let mut computer = Computer::from_string(input);
        computer.execute();
        assert_eq!(computer.get_output(), "4,6,3,5,6,3,5,2,1,0");
    }
}