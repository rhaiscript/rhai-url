# About `rhai-url`

[![License](https://img.shields.io/crates/l/rhai-url)](https://github.com/license/rhaiscript/rhai-url)
[![crates.io](https://img.shields.io/crates/v/rhai-url?logo=rust)](https://crates.io/crates/rhai-url/)
[![crates.io](https://img.shields.io/crates/d/rhai-url?logo=rust)](https://crates.io/crates/rhai-url/)
[![API Docs](https://docs.rs/rhai-url/badge.svg?logo=docs-rs)](https://docs.rs/rhai-url/)

This crate provides `url::Url` access for the [Rhai] scripting language.

## Usage

### `Cargo.toml`

```toml
[dependencies]
rhai-url = "0.0.4"
```

### [Rhai] script

```js
// create a new Url
let url = Url("http://example.com/?q=query");

// get the absolute url as a string
print(url.href); // print 'http://example.com/?q=query'
print(url.to_string()); // print 'http://example.com/?q=query'

// get the url query string, without the leading ?
print(url.query); // print 'q=query'

// get the url fragment
print(url.fragment); // print ''

// hash is an alias of fragment
print(url.hash); // print ''

// clear the query
url.query_clear();
print(url.query); // print ''

// remove a query key
url.query_remove("q");

// query_remove with no arguments will clear the query string
url.query_remove();

// adds a query key value pair into the query string
url.query_append("q", "name");

```

You can see an example on how to use those function in the [tests](tests/url.rs).

### Rust source

```rust
use rhai::{Engine, EvalAltResult};
use rhai::packages::Package;
use rhai_url::UrlPackage;
use url::Url;

fn main() -> Result<(), Box<EvalAltResult>> {
    // Create Rhai scripting engine
    let mut engine = Engine::new();

    // Create url package and add the package into the engine
    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    // Print the url
    let url = engine.eval::<Url>(r#"Url("http://test.dev/")"#)?;

    println!("{}", url);

    // Print the url string, equivalent of to_string()
    let href = engine.eval::<String>(r#"Url("http://test.dev/").href"#)?;

    println!("{}", href);

    Ok(())
}
```

## Features

|  Feature   | Default  | Description                                          |
| :--------: | :------: | ---------------------------------------------------- |
| `array`    | enabled  | Enables support for [Rhai] `Array`                   |
| `metadata` | disabled | Enables support for generating package documentation |

[Rhai]: https://rhai.rs