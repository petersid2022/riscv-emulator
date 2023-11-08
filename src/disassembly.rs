use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;

pub fn disassembly(dir: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let bin_data = fs::read(dir)?;
    let obj_file = object::File::parse(&*bin_data)?;
    if let Some(section) = obj_file.section_by_name(".text") {
        if let Ok(section_data) = section.data() {
            let mut instructions = Vec::new();
            for i in (0..section_data.len()).step_by(4) {
                if i + 4 <= section_data.len() {
                    let instruction = &section_data[i..i + 4];
                    let binary_string = format!(
                        "{:#034b}",
                        u32::from_le_bytes([
                            instruction[0],
                            instruction[1],
                            instruction[2],
                            instruction[3]
                        ])
                    );
                    let out = u32::from_str_radix(&binary_string[2..], 2)
                        .map_err(|e| format!("Failed to parse binary string: {}", e))?;
                    instructions.push(out);
                }
            }
            return Ok(instructions);
        } else {
            return Err("Failed to disassemble".into());
        }
    } else {
        return Err("Section not available".into());
    }
}
