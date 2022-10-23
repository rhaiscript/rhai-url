use rhai::plugin::*;

#[export_module]
pub mod url_module {
    use url::Url;

    #[rhai_fn(name = "Url", return_raw)]
    pub fn new(url: &str) -> Result<Url, Box<EvalAltResult>> {
        Url::parse(url).map_err(|e| Box::<EvalAltResult>::from(e.to_string()))
    }

    #[rhai_fn(global, get = "href", pure)]
    pub fn href(url: &mut Url) -> ImmutableString {
        url.to_string().into()
    }

    #[rhai_fn(global, get = "scheme", pure)]
    pub fn scheme(url: &mut Url) -> ImmutableString {
        url.scheme().into()
    }

    #[rhai_fn(global, set = "scheme", pure)]
    pub fn set_scheme(url: &mut Url, value: &str) {
        _ = url.set_scheme(value);
    }

    #[rhai_fn(global, get = "domain", pure)]
    pub fn domain(url: &mut Url) -> ImmutableString {
        url.domain().unwrap_or("").into()
    }

    #[rhai_fn(global, get = "path", pure)]
    pub fn path(url: &mut Url) -> ImmutableString {
        url.path().into()
    }

    #[rhai_fn(global, set = "path", pure)]
    pub fn set_path(url: &mut Url, value: &str) {
        url.set_path(value)
    }

    #[rhai_fn(global, get = "query", pure)]
    pub fn query(url: &mut Url) -> ImmutableString {
        url.query().unwrap_or("").into()
    }

    #[rhai_fn(global, set = "query", pure)]
    pub fn set_query(url: &mut Url, value: &str) {
        if value.len() > 0 {
            url.set_query(Some(value))
        } else {
            url.set_query(None)
        }
    }

    #[rhai_fn(global, set = "query", pure)]
    pub fn set_query_option(url: &mut Url, value: Option<&str>) {
        match value {
            Some(value) => url.set_query(Some(value)),
            None => url.set_query(None),
        }
    }

    #[rhai_fn(global, get = "fragment", pure)]
    pub fn fragment(url: &mut Url) -> ImmutableString {
        url.fragment().unwrap_or("").into()
    }

    #[rhai_fn(global, set = "fragment", pure)]
    pub fn set_fragment(url: &mut Url, value: &str) {
        if value.len() > 0 {
            url.set_fragment(Some(value))
        } else {
            url.set_fragment(None)
        }
    }

    #[rhai_fn(global, set = "fragment", pure)]
    pub fn set_fragment_option(url: &mut Url, value: Option<&str>) {
        match value {
            Some(value) => url.set_fragment(Some(value)),
            None => url.set_query(None),
        }
    }

    #[rhai_fn(global, get = "hash", pure)]
    pub fn hash(url: &mut Url) -> ImmutableString {
        url.fragment().unwrap_or("").into()
    }

    #[rhai_fn(global, set = "hash", pure)]
    pub fn set_hash(url: &mut Url, value: &str) {
        if value.len() > 0 {
            url.set_fragment(Some(value))
        } else {
            url.set_fragment(None)
        }
    }

    #[rhai_fn(global, set = "hash", pure)]
    pub fn set_hash_option(url: &mut Url, value: Option<&str>) {
        match value {
            Some(value) => url.set_fragment(Some(value)),
            None => url.set_query(None),
        }
    }

    /*************************************************************
     * Functions
     ************************************************************/

    #[rhai_fn(
        global,
        name = "query_clear",
        name = "query_delete",
        name = "query_remove",
        pure
    )]
    pub fn query_clear(url: &mut Url) {
        url.set_query(None)
    }

    #[rhai_fn(global, name = "query_delete", name = "query_remove", pure)]
    pub fn query_delete(url: &mut Url, key: &str) {
        let query: Vec<(String, String)> = url
            .query_pairs()
            .filter(|(name, _)| name != key)
            .map(|(name, value)| (name.into_owned(), value.into_owned()))
            .collect();

        url.query_pairs_mut().clear().extend_pairs(&query);

        // cleanup
        if let Some(q) = url.query() {
            if q.len() == 0 {
                url.set_query(None)
            }
        }
    }

    #[rhai_fn(global, name = "query_append", pure)]
    pub fn query_append(url: &mut Url, key: &str, value: &str) {
        url.query_pairs_mut().append_pair(key, value);
    }

    #[rhai_fn(global, name = "query_set", pure)]
    pub fn query_set(url: &mut Url, key: &str, value: &str) {
        query_delete(url, key);
        query_append(url, key, value);
    }

    #[rhai_fn(global, name = "query_get", pure)]
    pub fn query_get(url: &mut Url, key: &str) -> ImmutableString {
        match url.query_pairs().find(|(name, _)| name == key) {
            Some((_, value)) => ImmutableString::from(value.as_ref()),
            None => ImmutableString::from(""),
        }
    }

    #[rhai_fn(global, name = "query_gets", name = "query_getAll", pure)]
    pub fn query_gets(url: &mut Url, key: &str) -> Vec<ImmutableString> {
        url.query_pairs()
            .filter(|(name, _)| name == key)
            .map(|(_, value)| ImmutableString::from(value.as_ref()))
            .collect()
    }

    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(url: &mut Url) -> ImmutableString {
        url.to_string().into()
    }
}
