# Dependencies

Rust, make sure you install the nightly:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh -s -- --channel=nightly
```

PHP, `5.6.7`, make sure you build it with `--debug-enabled`

# Building the Rust part

```
$ cargo build
$ cp target/debug/librust_a_star-HEXSIGNATURE.dylib PATH/TO/YOUR/PHP/EXTENSIONS/librust_a_star.dylib
```

# Running

```
$ PATH/TO/YOUR/DEBUG/ENABLED/PHP/php src/rust_a_star.php
```

# Other helpful links

[Valgrind Output](https://gist.github.com/jaredonline/e1317b4fe95db20b9dba)
[cpp Output](https://gist.github.com/jaredonline/f17f95e1a4705d8d14ed)
