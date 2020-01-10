# Documentation
This repo hosts documentation on Github pages since docs.rs only has documentation for libraries, but untill a [bug](https://github.community/t5/GitHub-Actions/Github-action-not-triggering-gh-pages-upon-push/td-p/26869) in github is resolved it is not visible as pushes from GitHub actions don't trigger the Github Pages build. The documentation can be build from source in the source directory with:
```bash
> cargo doc --open --no-deps
```