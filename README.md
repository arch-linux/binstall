# binstall
A low-level CLI tool for installing/uninstalling custom bianaries to systems, keeps a basic record of installed bianaries for automated removal later.

***This software is a work in progress and requires elevated access to operate - use with great care.***

Unix-like and Darwin (MacOS) systems are currently supported. Windows Operability is on the todo-list. 

This package will enable tracking, installation and uninstallation of custom bianaries. The CLI maintains a simple record of what has been installed so it can be removed at a later time with no fuss.

##Installation.

```
cargo build --release
sudo binstall --install target/release/binstall
```

#Usage.
```
binstall -[option] filename
binstall -help
```

#Roadmap
- [x] Unix/Linux/Darwin Compatibility 
- [ ] Windows Compatibility
- [ ] Direct Compile 
- [ ] Direct from Cargo Install

#Changelog:


1.0.0 - Initial Release. Unix-like is fully supported.
