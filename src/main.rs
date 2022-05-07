use std::env;
mod help;
mod install;
mod list;
mod root_check;
mod uninstall;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut os = " ";
    const VERSION: &str = "0.0.1";

    // check for Unix/Linux
    if cfg!(windows) {
        os = "Windows";
    } else if cfg!(unix) {
        os = "Unix";
        //check root privliges
        root_check::main().expect("Error authenticating as root.");
        //initialize config.ini
        list::initialize_list(&os);
    }

    println!("BINstall v{VERSION} {os}");

    if os == "Windows" {
        println!(
            "This application does not support Windows at this time, please check back later."
        );
        process::exit(0x0100);
    }

    // check if there are no command line arguments

    let possible_commands = vec!["-help", "-install", "-uninstall", "-list"];

    if args.len() == 1 {
        println!("No command line arguments specified, showing -help.");
        help::help_screen();
    } else {
        let temp_cmd: &str = &args[1].to_string();
        if possible_commands.contains(&temp_cmd) {
            // command was found based on possible_commands
            match temp_cmd {
                "-help" => help::help_screen(),
                "-list" => list::fetch_list(&os),
                "-install" => install::main(&args[2]),
                "-uninstall" => uninstall::uninstall(&os, args[2].to_string()),
                &_ => print!(""),
            }
        } else {
            // no known command
            // clean up command for error message

            let mut doctored_command: String = String::from(temp_cmd);
            // determine if there is a '-' in front of the command.
            let operator = doctored_command.chars().nth(0).unwrap();
            if operator == '-' {
                doctored_command.remove(0);
            }
            println!("Unknown command {doctored_command}, pass -help for advice.");
        }
    }
}
