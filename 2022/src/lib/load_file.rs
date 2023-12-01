use std::fs;

pub fn load_data_file(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(format!("./data/{}", name))?;
    Ok(data)
}
