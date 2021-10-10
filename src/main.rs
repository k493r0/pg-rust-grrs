use structopt::StructOpt;
use anyhow::{Context, Result};


#[derive(StructOpt)]
struct Cli{
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let args = Cli::from_args();
    // let content = std::fs::read_to_string(&args.path)
    //     .expect("Could not read file");
    // for line in content.lines(){
    //     if line.contains(&args.pattern){
    //         println!("{}",line);
    //     }
    // }
    // let result = std::fs::read_to_string("test.txt");
    // match result {
    //     Ok(content) => { content;}
    //     Err(error) => { return Err(error.into()); }
    //
    // }
    // println!("File content: {}", content);
    // Ok(())
    let path = "a.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path))?;
    println!("File content: {}", content);
    Ok(())
}
