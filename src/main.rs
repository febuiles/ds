use std::env;
use std::process;

fn print_help() {
    println!("print_help");
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let argc = args.len();

    match argc {
        0...1 => {
            print_help();
            process::exit(0);
        }
        _ => {
            let command = args[3];
            let options = args[4];
    println!("{} {}", command, options);
            match command {
                "start" => start(options),
                "stop" => stop(options),
                "restart" => restart(options),
                "info" => info(options),
                "enabled" => enabled(),
                "disabled" => disabled(),
            }

        }
    }

}
