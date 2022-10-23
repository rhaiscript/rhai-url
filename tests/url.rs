use rhai::{packages::Package, Engine, EvalAltResult, ImmutableString};

use rhai_url::UrlPackage;
use url::Url;

#[test]
fn test() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let package = UrlPackage::new();
    package.register_into_engine(&mut engine);

    assert!(engine.eval::<Url>(r#"Url("")"#).is_err());

    assert!(engine.eval::<Url>(r#"Url("http://test.dev/")"#).is_ok());

    assert_eq!(
        ImmutableString::from("http://test.dev/"),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/").href"#)?
    );

    assert_eq!(
        ImmutableString::from("http://test.dev/"),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/").to_string()"#)?
    );

    assert_eq!(
        ImmutableString::from("/test_path"),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/test_path").path"#)?
    );

    assert_eq!(
        ImmutableString::from("http"),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/test_path").scheme"#)?
    );

    assert_eq!(
        ImmutableString::from("fragment"),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/test_path#fragment").fragment"#)?
    );

    assert_eq!(
        ImmutableString::from("fragment"),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/test_path#fragment").hash"#)?
    );

    assert_eq!(
        ImmutableString::from(""),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/test_path").fragment"#)?
    );

    assert_eq!(
        ImmutableString::from(""),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/test_path").query"#)?
    );

    assert_eq!(
        ImmutableString::from("q=query_string"),
        engine.eval::<ImmutableString>(r#"Url("http://test.dev/?q=query_string").query"#)?
    );

    assert_eq!(
        ImmutableString::from("http://test.dev/"),
        engine.eval::<ImmutableString>(
            r#"
            let url = Url("http://test.dev/?q=query_string");

            url.query_clear();

            url.href
        "#
        )?
    );

    assert_eq!(
        ImmutableString::from("http://test.dev/"),
        engine.eval::<ImmutableString>(
            r#"
            let url = Url("http://test.dev/?q=query_string");

            url.query_remove("q");

            url.href
        "#
        )?
    );

    assert_eq!(
        ImmutableString::from("http://test.dev/?b=c"),
        engine.eval::<ImmutableString>(
            r#"
            let url = Url("http://test.dev/?q=query_string&b=c");

            url.query_remove("q");

            url.href
        "#
        )?
    );

    assert_eq!(
        ImmutableString::from("http://test.dev/?b=c&query=name"),
        engine.eval::<ImmutableString>(
            r#"
            let url = Url("http://test.dev/?q=query_string&b=c");

            url.query_remove("q");
            url.query_append("query", "name");

            url.href
        "#
        )?
    );

    assert_eq!(
        ImmutableString::from("http://test.dev/?q=name"),
        engine.eval::<ImmutableString>(
            r#"
            let url = Url("http://test.dev/?q=query_string&q=c");

            url.query_set("q", "name");

            url.href
        "#
        )?
    );

    Ok(())
}
