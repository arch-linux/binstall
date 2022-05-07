use is_executable::IsExecutable;
use std::fs;
use std::path::Path;
use std::process;
mod list;

pub fn main(fromlocation: &String) {
    let mut bindir = "";

    if cfg!(windows) {
        // unimplemented
    } else if cfg!(unix) {
        //unix
        bindir = "Unix";
    }

    if bindir == "Unix".to_string() {
        if cfg!(target_os = "macos") {
            //setup /usr/local/bin as this does not exist by default in macOS for some reason.
            // also sometimes it does exist? so like idk what in the spaghetti is going on here
            match fs::create_dir("/usr/local/bin") {
                Ok(_garbage) => {
                    // do nothing because it worked.
                }
                Err(_) => {
                    // also do fucking nothing because macOS is inconsistently wierd.
                }
            }
        }
        //check to ensure given fromlocation is just a filename and not a full pwd.
        #[allow(unused_variables)]
        let last_element = fromlocation.split("/");
        let conf_last_element = &last_element.last().unwrap().to_string();
        // check if file exists
        let doesexist = Path::new(fromlocation).exists();
        if !doesexist {
            println!("Fatal:  specified file {fromlocation} does not exist, terminating.");
            process::exit(0x0100);
        } else {
            // file does exist
            let frompath = Path::new(fromlocation);
            if frompath.is_executable() {
                // good!
                let topath = format!("{}{}", "/usr/local/bin/", conf_last_element);
                let pre_existing_executible: bool = Path::new(&topath).exists();
                if pre_existing_executible {
                    // the filename already exists in bin, can't do anything about it. let the user know
                    println!("Fatal: {topath} already exists in /usr/local/bin, either rename your application or delete the offender.");
                } else {
                    match fs::copy(frompath, topath) {
                        Ok(_data) => {
                            list::install_list(&bindir, conf_last_element);
                        }
                        Err(_) => {
                            // there was an issue, termate the process
                            println!("Fatal: Failed to copy executible to /usr/local/bin/");
                            process::exit(0x0100);
                        }
                    }
                }
            } else {
                // bad!
                println!(
                    "Fatal: Provided filename {fromlocation} is NOT an executable, terminating."
                );
                process::exit(0x0100);
            }
        }
    } else {
    }
}
