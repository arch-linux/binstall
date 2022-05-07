#[allow(dead_code)]
pub fn help_screen() {
    println!("BINstall ");
    println!("BINstall will install/uninstall a (rust) application into bin on unix-like systems.");
    println!("binstall -[option] filename");
    println!("-help - displays this help menu");
    println!("-list - displays applications installed by BINstall");
    println!("-install [executable] - installs an applicaton to bin (requires root)");
    println!("-uninstall [executable]|[id] - removes an executable from bin directory and applicaiton list.");
    println!("Written by Christopher Allen, 2022");
}
