#[allow(unused_imports)]
use rhai::plugin::*;

#[export_module]
pub mod url_module {
    use url::Url;

    /// Creates a new Url.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/")
    /// ```
    #[rhai_fn(name = "Url", return_raw)]
    pub fn new(url: &str) -> Result<Url, Box<EvalAltResult>> {
        Url::parse(url).map_err(|e| Box::<EvalAltResult>::from(e.to_string()))
    }

    /// Gets the full Url, same as to_string().
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/")
    ///   let fullUrl = url.href // 'http://test.dev/'
    /// ```
    #[rhai_fn(global, get = "href", pure)]
    pub fn href(url: &mut Url) -> ImmutableString {
        url.to_string().into()
    }

    /// Gets the Url scheme such as 'http' or 'https'.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/")
    ///   let scheme = url.scheme // 'http'
    /// ```
    #[rhai_fn(global, get = "scheme", pure)]
    pub fn scheme(url: &mut Url) -> ImmutableString {
        url.scheme().into()
    }

    /// Sets the Url scheme.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/")
    ///   url.scheme = "https"
    ///
    ///   let scheme = url.scheme // 'https'
    ///   let fullUrl = url.href // 'https://test.dev/'
    /// ```
    #[rhai_fn(global, set = "scheme", pure)]
    pub fn set_scheme(url: &mut Url, value: &str) {
        _ = url.set_scheme(value);
    }

    /// Gets the Url domain.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/")
    ///   let domain = url.domain // 'test.dev'
    /// ```
    #[rhai_fn(global, get = "domain", pure)]
    pub fn domain(url: &mut Url) -> ImmutableString {
        url.domain().unwrap_or("").into()
    }

    /// Gets the Url path.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/path")
    ///   let path = url.path // '/path'
    /// ```
    #[rhai_fn(global, get = "path", pure)]
    pub fn path(url: &mut Url) -> ImmutableString {
        url.path().into()
    }

    /// Sets the Url path.
    #[rhai_fn(global, set = "path", pure)]
    pub fn set_path(url: &mut Url, value: &str) {
        url.set_path(value)
    }

    /// Gets the Url query string.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/?page=2")
    ///   let query = url.query // 'page=2'
    /// ```
    #[rhai_fn(global, get = "query", pure)]
    pub fn query(url: &mut Url) -> ImmutableString {
        url.query().unwrap_or("").into()
    }

    /// Sets the Url query string.
    #[rhai_fn(global, set = "query", pure)]
    pub fn set_query(url: &mut Url, value: &str) {
        if value.len() > 0 {
            url.set_query(Some(value))
        } else {
            url.set_query(None)
        }
    }

    /// Sets the Url query string.
    #[rhai_fn(global, set = "query", pure)]
    pub fn set_query_option(url: &mut Url, value: Option<&str>) {
        match value {
            Some(value) => url.set_query(Some(value)),
            None => url.set_query(None),
        }
    }

    /// Gets the Url fragment.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/?#row=4")
    ///   let fragment = url.fragment // 'row=4'
    /// ```
    #[rhai_fn(global, get = "fragment", pure)]
    pub fn fragment(url: &mut Url) -> ImmutableString {
        url.fragment().unwrap_or("").into()
    }

    /// Sets the Url fragment.
    #[rhai_fn(global, set = "fragment", pure)]
    pub fn set_fragment(url: &mut Url, value: &str) {
        if value.len() > 0 {
            url.set_fragment(Some(value))
        } else {
            url.set_fragment(None)
        }
    }

    /// Sets the Url fragment.
    #[rhai_fn(global, set = "fragment", pure)]
    pub fn set_fragment_option(url: &mut Url, value: Option<&str>) {
        match value {
            Some(value) => url.set_fragment(Some(value)),
            None => url.set_query(None),
        }
    }

    /// Gets the Url hash, alias of fragment.
    ///
    /// # Examples
    ///
    /// Rhai usage:
    ///
    /// ```js, rhai
    ///   let url = Url("http://test.dev/?#row=4")
    ///   let hash = url.hash // 'row=4'
    /// ```
    #[rhai_fn(global, get = "hash", pure)]
    pub fn hash(url: &mut Url) -> ImmutableString {
        url.fragment().unwrap_or("").into()
    }

    /// Sets the Url hash.
    #[rhai_fn(global, set = "hash", pure)]
    pub fn set_hash(url: &mut Url, value: &str) {
        if value.len() > 0 {
            url.set_fragment(Some(value))
        } else {
            url.set_fragment(None)
        }
    }

    /// Sets the Url hash.
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
