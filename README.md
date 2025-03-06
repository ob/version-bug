# Demonstrate bug in version-compare crate

Given the following versions:

```
7.6.24-15+43e3480
7.6.24-11+8a3cc70
7.6.24-1+6dd6c8e
7.6.24-rc2.10+dd726d8
7.6.24-rc2.5+347fb4d
7.6.24-rc2.3+647f389
7.6.24-rc2.2+35d5a3c
7.6.24-rc2
7.6.23-6+fc244ee
7.6.23-3+3c3c027
7.6.23
7.6.23-rc4.1+2868e74
7.6.23-rc4
7.6.23-rc3
7.6.23-rc2.16+ed0034f
7.6.23-rc2.15+3267aef
7.6.23-rc2.14+d48fcbc
7.6.23-rc2.13+04aefb8
7.6.23-rc2.10+58cd2e0
7.6.23-rc2.7+0115675
7.6.23-rc2
7.6.23-rc1.4+54a763d
7.6.23-rc1.3+0b005b3
7.6.23-rc1.2+5fb92e5
7.6.23-rc1.1+ffb012b
7.6.23-rc1
7.6.22-3+5d2ffb4
7.6.22-2+f4f7caf
7.6.22-1+a49aab4
7.6.22
7.6.21-12+c224cf1
7.6.21
7.6.20
7.6.20-rc2
7.6.18
7.6.16
7.6.15
6.12.25
```

Trying to sort them with the [version-compare](https://crates.io/crates/version-compare) crate, produces:

```rust
 cargo run
   Compiling version-test v0.1.0 (/Users/obonilla/o/version-test)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/version-test`

thread 'main' panicked at library/core/src/slice/sort/shared/smallsort.rs:865:5:
user-provided comparison function does not correctly implement a total order
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

