use rhai::{packages::Package, Engine, EvalAltResult};

use rhai_url::UrlPackage;
use url::Url;

/**
 * Tests Url()
 */
#[test]
fn test_constructors() {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    // it should be an error on empty url
    assert!(
        engine.eval::<Url>(r#"Url("")"#).is_err(),
        "it should be an error on empty url"
    );

    // it should be ok on a valid url
    assert!(
        engine.eval::<Url>(r#"Url("http://test.dev/")"#).is_ok(),
        "it should be ok on a valid url"
    );

    // it should be ok on a valid IPv4 url
    assert!(
        engine.eval::<Url>(r#"Url("http://127.0.0.1/")"#).is_ok(),
        "it should be ok on a valid IPv4 url"
    );

    // it should be ok on a valid IPv6 url
    assert!(
        engine.eval::<Url>(r#"Url("http://[::1]/")"#).is_ok(),
        "it should be ok on a valid IPv6 url"
    );
}

/**
 * Tests .href, .to_string(), .to_debug()
 */
#[test]
fn test_href_to_string_to_debug() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev/").href"#)?,
        "http://test.dev/",
        "it should return the absolute url"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev/").to_string()"#)?,
        "http://test.dev/",
        "it should return the absolute url"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev/").to_debug()"#)?,
        "http://test.dev/",
        "it should return the absolute url"
    );

    Ok(())
}

/**
 * Tests scheme getter and setter
 */
#[test]
fn test_scheme_getter_setter() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev").scheme"#)?,
        "http",
        "it should return 'http'"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("custom-scheme://test.dev/").scheme"#)?,
        "custom-scheme",
        "it should return 'custom-scheme'"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/");
        url.scheme = "https";
        url.scheme
        "#
        )?,
        "https",
        "it should set the scheme to 'https'"
    );

    // NOTE: it might be confusing, however this is actually the implementation from url::Url

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/");
        url.scheme = "custom-scheme";
        url.scheme
        "#
        )?,
        "http",
        "it should refuse to set a 'custom-scheme' when the current scheme is 'http'"
    );

    Ok(())
}

/**
 * Tests domai getter
 */
#[test]
fn test_domain_getter_as_domain_has_no_setter() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev").domain"#)?,
        "test.dev",
        "it should return a the domain"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://127.0.0.1").domain"#)?,
        "",
        "it should return an empty string when the domain is an IPv4"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev:1234").domain"#)?,
        "test.dev",
        "it should not include the port"
    );

    Ok(())
}

/**
 * Tests path getter and setter
 */
#[test]
fn test_path_getter_setter() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev").path"#)?,
        "/",
        "it should return a forward slash on url without end slash"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev/").path"#)?,
        "/",
        "it should return a forward slash on url with no path"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev/test_path").path"#)?,
        "/test_path",
        "it should return the path with a leading forward slash"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/test_path");
        url.path = "new-path";
        url.path
        "#
        )?,
        "/new-path",
        "it should return the newly assigned path"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/test_path");
        url.path = "";
        url.path
        "#
        )?,
        "/",
        "it should return the newly assigned path empty path as a forward slash"
    );

    Ok(())
}

/**
 * Tests query getter and setter
 */
#[test]
fn test_query_getter_setter() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev").query"#)?,
        "",
        "it should return an empty string when there is no query string"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev?").query"#)?,
        "",
        "it should return an empty string when there is no query string but '?' is present"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev?test_query").query"#)?,
        "test_query",
        "it should return the query without the leading '?'"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.query = "q=test";
        url.query
        "#
        )?,
        "q=test",
        "it should return the newly assigned query"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/test_query");
        url.query = "";
        url.query
        "#
        )?,
        "",
        "it should return the newly assigned empty query"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.query = "";
        url.href
        "#
        )?,
        "http://test.dev/path",
        "it should not add the '?' when the new query is empty"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.query = "q=test";
        url.href
        "#
        )?,
        "http://test.dev/path?q=test",
        "it should only set the query string"
    );

    Ok(())
}

/**
 * Tests query getter and setter
 */
#[test]
fn test_query_clear_delete_remove() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/?query");
        url.query_clear();
        url.query
        "#
        )?,
        "",
        "it should clear the query"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/?query");
        url.query_clear();
        url.href
        "#
        )?,
        "http://test.dev/",
        "it should remove the '?'"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/?query");
        url.query_delete();
        url.query
        "#
        )?,
        "",
        "it should delete the query when no argument is passed"
    );

    // query_remove is an alias of query_delete

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/?query");
        url.query_remove();
        url.query
        "#
        )?,
        "",
        "it should delete the query when no argument is passed"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/?q=test&b=preserve");
        url.query_delete("q");
        url.query
        "#
        )?,
        "b=preserve",
        "it should only delete the specified query key"
    );

    Ok(())
}

/**
 * Tests fragment getter and setter
 */
#[test]
fn test_fragment_getter_setter() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev").fragment"#)?,
        "",
        "it should return an empty string when there is no fragment string"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev#").fragment"#)?,
        "",
        "it should return an empty string when there is no fragment string but '#' is present"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev#test_fragment").fragment"#)?,
        "test_fragment",
        "it should return the fragment without the leading '#'"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.fragment = "test";
        url.fragment
        "#
        )?,
        "test",
        "it should return the newly assigned fragment"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/");
        url.fragment = "";
        url.fragment
        "#
        )?,
        "",
        "it should return the newly assigned empty fragment"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.fragment = "";
        url.href
        "#
        )?,
        "http://test.dev/path",
        "it should not add the '#' when the new fragment is empty"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.fragment = "test";
        url.href
        "#
        )?,
        "http://test.dev/path#test",
        "it should only set the fragment string"
    );

    Ok(())
}

/**
 * Tests hash getter and setter, hash and fragment should behave identically
 */
#[test]
fn test_hash_getter_setter_as_fragment_alias() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev").hash"#)?,
        "",
        "it should return an empty string when there is no fragment string"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev#").hash"#)?,
        "",
        "it should return an empty string when there is no fragment string but '#' is present"
    );

    assert_eq!(
        engine.eval::<String>(r#"Url("http://test.dev#test_fragment").hash"#)?,
        "test_fragment",
        "it should return the fragment without the leading '#'"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.hash = "test";
        url.hash
        "#
        )?,
        "test",
        "it should return the newly assigned fragment"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/");
        url.hash = "";
        url.hash
        "#
        )?,
        "",
        "it should return the newly assigned empty fragment"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.hash = "";
        url.href
        "#
        )?,
        "http://test.dev/path",
        "it should not add the '#' when the new fragment is empty"
    );

    assert_eq!(
        engine.eval::<String>(
            r#"
        let url = Url("http://test.dev/path");
        url.hash = "test";
        url.href
        "#
        )?,
        "http://test.dev/path#test",
        "it should only set the fragment string"
    );

    Ok(())
}
