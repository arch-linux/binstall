extern crate ini;
use ini::Ini;

const UNIXCONFIGLOCATION: &str = "/etc/binstall/config.ini";

pub fn install_list(os: &&str, execname: &String) {
    if os.to_string() == "Unix" {
        // pull current numbers from the library.
        let mut i = Ini::load_from_file(UNIXCONFIGLOCATION).unwrap();
        let current_numbers = i.get_from(Some("bin"), "entries");
        let installed_programs: i64 = current_numbers.unwrap().to_string().parse().unwrap();
        let newid: i64 = installed_programs + 1;
        i.with_section(Some("bin"))
            .set("entries", newid.to_string());
        let locationofnewexec = format!("{}{}", "/usr/local/bin/", execname);
        i.with_section(Some(newid.to_string()))
            .set("name", &execname.to_string())
            .set("location", locationofnewexec);
        i.write_to_file(UNIXCONFIGLOCATION).unwrap();
    }
}
