#[allow(dead_code)]
#[allow(unused_variables)]
extern crate ini;
use ini::Ini;
use std::fs;

const UNIXCONFIGDIRECTORY: &str = "/etc/binstall/";
const UNIXCONFIGLOCATION: &str = "/etc/binstall/config.ini";

pub fn fetch_list(os: &&str) {
    if os.to_string() == "Unix" {
        let i = Ini::load_from_file(UNIXCONFIGLOCATION).unwrap();
        let current_applications = i.get_from(Some("bin"), "entries");
        let current_applications: i64 = current_applications.unwrap().parse().unwrap();
        println!("Listing Applications Currently Installed.");
        for x in 0..current_applications {
            let additive: i64 = x + 1;
            let appname = i.get_from(Some(additive.to_string()), "name");
            if appname.is_none() {
                // do nothing
            } else {
                println!("ID:{} - NAME:{}", x + 1, appname.unwrap().to_string());
            }
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn initialize_list(os: &&str) {
    if os.to_string() == "Unix" {
        //check if the file exists
        match Ini::load_from_file(UNIXCONFIGLOCATION) {
            Ok(data) => {
                // found configuration file. do nothing.
            }
            Err(_) => {
                println!("First startup, creating database: {UNIXCONFIGLOCATION}");
                match fs::create_dir(UNIXCONFIGDIRECTORY) {
                    Ok(data) => {}
                    Err(_) => {
                        println!("Could not create configuration directory at {UNIXCONFIGDIRECTORY}, maybe it already exists?");
                    }
                }

                let mut conf = Ini::new();
                conf.with_section(Some("bin")).set("entries", "0");
                conf.write_to_file(UNIXCONFIGLOCATION).unwrap();
            }
        }
    }
}
