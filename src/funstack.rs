use crate::cli::Args;

pub fn run(args: Vec<String>) -> Result<(), String> {
    
    let args: Args = Args::parse(args)?;
    
    Ok(())
}
