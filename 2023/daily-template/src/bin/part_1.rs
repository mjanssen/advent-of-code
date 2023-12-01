use {{crate_name}}::part_1::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_str!("../../input_1.txt");
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
