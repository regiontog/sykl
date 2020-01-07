# Installing
If you are on x86-64 windows or linux you can download a prebuilt binary from [github releases](https://github.com/regiontog/sykl/releases)

# Running
The application takes no arguments and writes to stdout, so it should be ran from the command line.

# Compiling
If you have a [rust toolchain](https://rustup.rs/) installed you can download and build the application with
```bash
> cargo install sykl
> sykl
NAME                           AVAILABLE DOCKS   AVAILABLE BIKES
Botanisk Hage vest             26                0
Linaaes gate                   21                0
Suhms gate                     21                0
Oslo Hospital                  15                0
Arkitekt Rivertz Plass         30                0
Grenseveien                    24                1
John Colletts plass            25                0
Hoffsveien                     18                0
Nydalen                        18                0
...
```

Alternatively you can download the source from github and in the source directory run
```bash
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/sykl`
NAME                           AVAILABLE DOCKS   AVAILABLE BIKES
Botanisk Hage vest             26                0
Linaaes gate                   21                0
Suhms gate                     21                0
Oslo Hospital                  15                0
Arkitekt Rivertz Plass         30                0
Grenseveien                    24                1
John Colletts plass            25                0
Hoffsveien                     18                0
Nydalen                        18                0
...
```

# Documentation
This repo has a Github pages with the documentation, but untill a [bug](https://github.community/t5/GitHub-Actions/Github-action-not-triggering-gh-pages-upon-push/td-p/26869) in github is resolved it is not visible as pushes from GitHub actions don't trigger the Github Pages build. The documentation can be build from source in the source directory with:
```bash
> cargo doc --document-private-items --open
```
