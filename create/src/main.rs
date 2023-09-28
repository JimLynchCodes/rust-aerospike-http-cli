use std::error::Error;

mod create;

fn main() -> Result<(), Box<dyn Error>> {
    let response = create::insert_hardcoded()?;

    println!("result: {:#?}", response);

    Ok(())
}
