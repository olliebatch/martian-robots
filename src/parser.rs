use crate::mission_instructions::Coordinates;
use anyhow::anyhow;
use std::error;
use std::str::FromStr;

fn parse_input_to_command(commands: &str) -> Result<(), Box<dyn error::Error>> {
    let mut lines_of_instruction = commands.lines();
    //todo add a check here to actually check the number of lines etc
    let coords = lines_of_instruction.next();
    // match until cover the lines so that can unwrap.
    let coordinates = match coords {
        None => Err(anyhow!("No Upper right point provided.")),
        Some(values) => Ok(Coordinates::from_str(values)?),
    }?;

    Ok(())
}
