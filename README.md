# RedisPP

RedisPP (RePP) is a [Redis](https://redis.io/) module that adds Pretty Printing capabilities.

## Primary features:

* Colorized and formatted JSON output for: Hashes
* ASCII Table output for: Hashes

## Commands

### `PP.J` - Pretty-Print as JSON

Use `PP.J` to pretty-print a Redis Hash as colorized JSON:

```
PP.J hashy.hash:001
```

![PP.J](/docs/screenshots/pp.j.hash.png "PP.J w/ Hash")

### `PP.T` - Pretty-Print as ASCII Tables

Use `PP.T` to pretty-print a Redis Hash as an ASCII table:

```
PP.T hashy.hash:001
```

![PP.T](/docs/screenshots/pp.t.hash.png "PP.T w/ Hash")

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