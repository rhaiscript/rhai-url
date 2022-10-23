# `rhai-url`

This crate provides `url::Url` access for the [Rhai] scripting language.

## Usage

### `Cargo.toml`

```toml
[dependencies]
rhai-url = "0.0.1"
```

### [Rhai] script

```js
let url = Url("http://example.com/?q=query");

print(url.href); // print 'http://example.com/?q=query'
print(url.to_string()); // print 'http://example.com/?q=query'

print(url.query); // print 'q=query'

// fragment and hash are aliases
print(url.fragment); // print ''
print(url.hash); // print ''

url.query_clear();
print(url.query); // print ''

url.query_remove("q");
url.query_append("q", "name");
```

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
