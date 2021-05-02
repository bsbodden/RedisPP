# RedisPP

RedisPP (RePP) is a [Redis](https://redis.io/) module that adds Pretty Printing capabilities.

## Primary features:

* Colorized and formatted JSON output for: Hashes

## Commands

### `PP.J` - Pretty-Print as JSON

Use `PP.J` to pretty-print a Redis Hash as colorized JSON:

```
PP.J hashy.hash:001
```

![PP.J](/docs/screenshots/pp.j.hash.png "PP.J w/ Hash")

## Build

Make sure you have Rust installed:
https://www.rust-lang.org/tools/install

Then, build as usual:

```bash
cargo build
```

## Run

### Linux

```
redis-server --loadmodule ./target/release/librepp.so
```

### Mac OS

```
redis-server --loadmodule ./target/debug/librepp.dylib
```

## License

TBD