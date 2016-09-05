use std::env;
use std::process;

fn exit_with_usage() {
    println!("print_usage");
    process::exit(1);
}

fn start(service: String) {
    println!("{}", service);
}

fn stop(service: String) {
    println!("{}", service);
}

fn restart(service: String) {
    println!("{}", service);
}

fn info(service: String) {
    println!("{}", service);
}

fn enabled() {
}

fn disabled() {
}

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
