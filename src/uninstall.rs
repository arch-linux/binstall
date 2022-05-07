use ini::Ini;
use std::char;
use std::fs;

const UNIXCONFIGLOCATION: &str = "/etc/binstall/config.ini";

fn uninstall_by_id(os: &str, id: i64) {
    if os == "Unix" {
        let mut i = Ini::load_from_file(UNIXCONFIGLOCATION).unwrap();
        let amatch = i.get_from(Some(id.to_string()), "name");
        if !amatch.is_none() {
            // found a match
            erase_from_disk(os, &amatch.unwrap().to_string());
            i.delete(Some(id.to_string()));
            i.write_to_file(UNIXCONFIGLOCATION)
                .expect("Fatal: Failed to write DB");
            println!("Process Completed!");
        } else {
            // did not find a match
            println!("Fatal: Did not find an ID match for {id}, please use -list to find the id or name you are looking for.");
            println!("I think your executible uses a name of just integers, please reference its ID in this case.");
        }
    }
}

fn uninstall_by_name(os: &str, name: String) {
    if os == "Unix" {
        let i = Ini::load_from_file(UNIXCONFIGLOCATION).unwrap();
        for (sec, prop) in i.iter() {
            for (k, v) in prop.iter() {
                if k == "name" && v == name {
                    let mut newmet = i.clone();
                    newmet.delete(Some(sec.unwrap().to_string()));
                    newmet
                        .write_to_file(UNIXCONFIGLOCATION)
                        .expect("Fatal: Failed to write DB");
                    erase_from_disk(os, &v.to_string());
                    break;
                }
            }
        }
    }
}

pub fn uninstall(os: &str, callsign: String) {
    if callsign.chars().all(char::is_numeric) {
        // this is most likely an ID;
        uninstall_by_id(os, callsign.parse().unwrap());
    } else {
        // this is most likely a Name;
        uninstall_by_name(os, callsign);
    }
}

fn erase_from_disk(os: &str, filename: &String) {
    if os == "Unix" {
        fs::remove_file(format!("{}{}", "/usr/local/bin/", filename))
            .expect("Fatal: Failed to remove executible from disk.");
    }
}
