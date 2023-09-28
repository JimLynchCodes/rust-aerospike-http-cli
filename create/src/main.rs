use std::error::Error;

mod create;

fn main() -> Result<(), Box<dyn Error>> {
    
    let response = create::get_some_json()?;

    println!("dynamic_json: {:#?}", response);

    let struct_deserialised_json = create::get_some_json()?;

    println!("struct_deserialised_json: {:#?}", struct_deserialised_json);

    Ok(())
}
