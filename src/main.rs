use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        exit_with_usage();
    }

    match argc {
        2 => {
            let command = args[1];
            println!("{}", command);
        },
        3 => {
            let command = args[1];
            let options = args[2];

            match &*command {
                "start" => start(options),
                "stop" => stop(options),
                "restart" => restart(options),
                "info" => info(options),
                "enabled" => enabled(),
                "disabled" => disabled(),
                _ => exit_with_usage(),
            }

        },
        _ => {
            exit_with_usage();
        }
    }
}
