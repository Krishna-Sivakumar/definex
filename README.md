# Define X
A terminal-based dictionary for MacOS.

There's no good dictionary database for free online, so I just parsed MacOS' native dictionary :P

Make a pull request if you feel like there's something that needs to be added.

### Usage
```shell
usage: definex [word]
```

### To Build
```shell
cargo build --release
```

### To Run
```shell
cargo run --release [word]
(or)
./target/release/definex [word] (post build)
```

### To do/fix
1. CFStrings are still FFI unsafe, need to be changed to CFStringRefs
2. There's a RAII bug on CoreServices' side sometimes
3. Select specific headings from definitions

### Credits
[kornelski/core-services](https://github.com/kornelski/core-services): to help understand core services FFI bindings
