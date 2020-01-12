# Installing
If you are on x86-64 windows or linux you can download a prebuilt binary from [github releases](https://github.com/regiontog/sykl/releases/tag/sykl-0.1.1). The binaries here are _not_ installers, but the program itself.

# Running
The application takes no arguments and writes to stdout, so it should be ran from the command line.

# Compiling
You can download the source from github and in the source directory run
```bash
> cargo run --bin sykl
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/sykl`
NAME                              AVAILABLE DOCKS   AVAILABLE BIKES
Botanisk Hage vest      26                                0
Linaaes gate                  21                                0
Suhms gate                   21                                0
Oslo Hospital                15                                0
Arkitekt Rivertz Plass   30                                0
Grenseveien                  24                                1
John Colletts plass        25                                0
Hoffsveien                     18                                0
Nydalen                         18                                0
...
```
