use structopt::StructOpt;
use anyhow::{Context, Result};


#[derive(StructOpt)]
struct Cli{
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())

    // #[test]
    // fn find_a_match(){
    //     let mut result = Vec::new();
    //     grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    //     assert_eq!(result, b"lorem ipsum\n");
    // }



    // let result = std::fs::read_to_string("test.txt");
    // match result {
    //     Ok(content) => { content;}
    //     Err(error) => { return Err(error.into()); }
    //
    // }
    // println!("File content: {}", content);
    // Ok(())
    // let path = "a.txt";
    // let content = std::fs::read_to_string(path)
    //     .with_context(|| format!("Could not read file `{}`", path))?;
    // println!("File content: {}", content);
    // Ok(())

    // use std::io::{self, Write};
    // let stdout = io::stdout();
    // let mut handle = io::BufWriter::new(stdout);
    // writeln!(handle, "Foo: {}", 42);


}
