use std::env;
use std::process;
fn main(){
    let _args: Vec<String> = env::args().collect();

    let parsed_args: Args = Args::parse_args(&_args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });
    println!("Init Command: {}", parsed_args.init);
    println!("QUery: {}", parsed_args.command)
}

struct Args{
    init: String,
    command: String,
}

impl Args{
    fn parse_args(args:&[String]) -> Result<Args, &str>{
        if args.len() < 3{
            return Err("You need to enter more arguments.")
        }
        let _init = args[1].clone();
        let _command = args[2].clone();

        Ok(Args{init: _init, command: _command})
    }

}