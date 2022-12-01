use crate::lib::load_file::load_data_file;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let _data = load_data_file("day1.txt")?;
    Ok(())
}
