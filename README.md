# RedisPP

RedisPP (RePP) is a [Redis](https://redis.io/) module that adds Pretty Printing capabilities.

## Primary features:

* Colorized and formatted JSON output for: Hashes
* ASCII Table output for: Hashes
* CSV output for: Hashes
* HTML Table output for: Hashes

## Commands

### `PP.J` - Pretty-Print as JSON

Use `PP.J` to pretty-print a Redis Hash as colorized JSON:

```
PP.J hashy.hash:001
```

![PP.J](/docs/screenshots/pp.j.hash.png "PP.J w/ Hash")

Use `PP.J` to pretty-print a Redis List as colorized JSON, it uses `LRANGE list 0 -1` by
default:

```
PP.J listy.list:001
```

![PP.J](/docs/screenshots/pp.j.list.png "PP.J w/ List")

Use `PP.J` to pretty-print a Redis Set as colorized JSON, it uses `SMEMBERS` by
default:

```
PP.J setty.set:001
```

![PP.J](/docs/screenshots/pp.j.set.png "PP.J w/ Set")

### `PP.T` - Pretty-Print as ASCII Tables

Use `PP.T` to pretty-print a Redis Hash as an ASCII table:

```
PP.T hashy.hash:001
```

![PP.T](/docs/screenshots/pp.t.hash.png "PP.T w/ Hash")

Use `PP.T` to pretty-print a Redis List as an ASCII table, it uses `LRANGE list 0 -1` by
default:

```
PP.T listy.list:001
```

![PP.T](/docs/screenshots/pp.t.list.png "PP.T w/ List")

Use `PP.T` to pretty-print a Redis Set as an ASCII table, it uses `SMEMBERS` by
default:

```
PP.T setty.set:001
```

![PP.T](/docs/screenshots/pp.t.set.png "PP.T w/ Set")

### `PP.C` - Print as CSV

Use `PP.C` to print a Redis Hash as CSV:

```
PP.C hashy.hash:001
```

![PP.C](/docs/screenshots/pp.c.hash.png "PP.C w/ Hash")

Use `PP.C` to pretty-print a Redis List as CSV, it uses `LRANGE list 0 -1` by
default:

```
PP.C listy.list:001
```

![PP.C](/docs/screenshots/pp.c.list.png "PP.C w/ List")

Use `PP.C` to pretty-print a Redis Set as CSV, it uses `SMEMBERS` by
default:

```
PP.C setty.set:001
```

![PP.C](/docs/screenshots/pp.c.set.png "PP.C w/ Set")

### `PP.H` - Print as HTML snippet

Use `PP.H` to print a Redis Hash as HTML (HTML Table):

```
PP.H hashy.hash:001
```

![PP.H](/docs/screenshots/pp.h.hash.png "PP.H w/ Hash")

Use `PP.H` to pretty-print a Redis List as HTML, it uses `LRANGE list 0 -1` by
default and it generates an HTML unordered list `<ol><li>`:

```
PP.H listy.list:001
```

![PP.H](/docs/screenshots/pp.h.list.png "PP.H w/ List")

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