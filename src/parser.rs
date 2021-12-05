pub enum Opcode {
    Inp,
    Add,
    Sub,
    Dup,
    Cond,
    Gotou(usize),
    Outn,
    Outa,
    Rol,
    Swap,
    Mul,
    Div,
    Pop,
    Push(usize),
    Ror,
    Nop,
}


pub fn parse(file: &str) -> Result<Vec<Opcode>, String> {
    let code = std::fs::read_to_string(file);

    if let Ok(code_string) = code {

        let lines = code_string.split("\n").collect::<Vec<&str>>();

        let mut parsed = vec![];

        let mut skip_next = false;
        for (i, line) in lines.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let line = String::from(*line);

            match line.len() {
                9 => parsed.push(Opcode::Inp),
                10 => parsed.push(Opcode::Add),
                11 => parsed.push(Opcode::Sub),
                12 => parsed.push(Opcode::Dup),
                13 => parsed.push(Opcode::Cond),
                14 => {
                    if i + 1 == lines.len() {
                        return Err(String::from("gotou must have a line under it"));
                    }

                    parsed.push(Opcode::Gotou(lines[i + 1].len()));
                    parsed.push(Opcode::Nop);

                    skip_next = true;
                }
                15 => parsed.push(Opcode::Outn),
                16 => parsed.push(Opcode::Outa),
                17 => parsed.push(Opcode::Rol),
                18 => parsed.push(Opcode::Swap),
                20 => parsed.push(Opcode::Mul),
                21 => parsed.push(Opcode::Div),
                23 => parsed.push(Opcode::Pop),
                25 => {
                    if i + 1 == lines.len() {
                        return Err(String::from("push must have a line under it"));
                    }

                    parsed.push(Opcode::Push(lines[i + 1].len()));
                    parsed.push(Opcode::Nop);

                    skip_next = true;
                }

                27 => parsed.push(Opcode::Ror),

                _ => return Err(String::from("Invalid opcode"))
            }
        }

        Ok(parsed)
    } else {
        Err(String::from("Could not open specified file"))
    }

}