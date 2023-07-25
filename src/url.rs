#[allow(unused_imports)]
use rhai::plugin::*;

#[export_module]
pub mod url_module {
    use url::Url;

    /// Creates a new Url.
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/")
    /// ```
    #[rhai_fn(name = "Url", return_raw)]
    pub fn new(url: &str) -> Result<Url, Box<EvalAltResult>> {
        Url::parse(url).map_err(|e| Box::<EvalAltResult>::from(e.to_string()))
    }

    /// Gets the full Url, same as to_string().
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/")
    /// let fullUrl = url.href // 'http://test.dev/'
    /// ```
    #[rhai_fn(global, get = "href", pure)]
    pub fn href(url: &mut Url) -> ImmutableString {
        url.to_string().into()
    }

    /// Gets the Url scheme such as 'http' or 'https'.
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/")
    /// let scheme = url.scheme // 'http'
    /// ```
    #[rhai_fn(global, get = "scheme", pure)]
    pub fn scheme(url: &mut Url) -> ImmutableString {
        url.scheme().into()
    }

    /// Sets the Url scheme.
    ///
    /// Be aware as this method will refuse to change the scheme under the following circumstances:
    ///
    /// * If the new scheme is not in `[a-zA-Z][a-zA-Z0-9+.-]+`
    /// * If this URL is cannot-be-a-base and the new scheme is one of
    ///   `http`, `https`, `ws`, `wss` or `ftp`
    /// * If either the old or new scheme is `http`, `https`, `ws`,
    ///   `wss` or `ftp` and the other is not one of these
    /// * If the new scheme is `file` and this URL includes credentials
    ///   or has a non-null port
    /// * If this URL's scheme is `file` and its host is empty or null
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/")
    /// url.scheme = "https"
    ///
    /// let scheme = url.scheme // 'https'
    /// let fullUrl = url.href // 'https://test.dev/'
    /// ```
    #[rhai_fn(global, set = "scheme", pure)]
    pub fn set_scheme(url: &mut Url, value: &str) {
        _ = url.set_scheme(value);
    }

    /// Gets the Url domain.
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/")
    /// let domain = url.domain // 'test.dev'
    /// ```
    #[rhai_fn(global, get = "domain", pure)]
    pub fn domain(url: &mut Url) -> ImmutableString {
        url.domain().unwrap_or("").into()
    }

    /// Gets the Url path.
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/path")
    /// let path = url.path // '/path'
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
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?page=2")
    /// let query = url.query // 'page=2'
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
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?#row=4")
    /// let fragment = url.fragment // 'row=4'
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
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?#row=4");
    /// let hash = url.hash; // 'row=4'
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
    /// Clear the query string.
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?q=query&b=1");
    ///
    /// url.query_clear();
    ///
    /// url == "http://test.dev/"
    /// ```
    pub fn query_clear(url: &mut Url) {
        url.set_query(None)
    }

    /// Delete a key from the query
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?q=query&b=1");
    ///
    /// url.query_delete("q"); // or
    /// url.query_remove("q");
    ///
    /// url == "http://test.dev/?b=1"
    /// ```
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

    /// Sets a query key
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?q=query&b=1");
    ///
    /// url.query_set("q", "new-query"); // or
    /// url.query_remove("q");
    ///
    /// url == "http://test.dev/?q=new-query&b=1"
    /// ```
    #[rhai_fn(global, name = "query_set", pure)]
    pub fn query_set(url: &mut Url, key: &str, value: &str) {
        query_delete(url, key);
        query_append(url, key, value);
    }

    /// Gets a query value for the specified key, it will return the first value found
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?q=query&b=1");
    ///
    /// url.query_get("q"); // "query"
    /// url.query_get("b"); // "1"
    /// ```
    #[rhai_fn(global, name = "query_get", pure)]
    pub fn query_get(url: &mut Url, key: &str) -> ImmutableString {
        match url.query_pairs().find(|(name, _)| name == key) {
            Some((_, value)) => ImmutableString::from(value.as_ref()),
            None => ImmutableString::from(""),
        }
    }

    /// Gets a list of values for the specified key
    ///
    /// Not available under `no_index`.
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/?q=query&q=second-query");
    ///
    /// url.query_get("q"); // ["query", "second-query"]
    /// ```
    #[rhai_fn(global, name = "query_gets", name = "query_getAll", pure)]
    #[cfg(not(feature = "no_index"))]
    pub fn query_gets(url: &mut Url, key: &str) -> rhai::Array {
        url.query_pairs()
            .filter(|(name, _)| name == key)
            .map(|(_, value)| ImmutableString::from(value.as_ref()).into())
            .collect()
    }

    /// Get the absolute url as a string
    ///
    /// ### Example
    ///
    /// ```js
    /// let url = Url("http://test.dev/path");
    ///
    /// url.to_string(); // "http://test.dev/path"
    /// url.to_debug(); // "http://test.dev/path"
    /// ```
    #[rhai_fn(global, name = "to_string", name = "to_debug", pure)]
    pub fn to_string(url: &mut Url) -> ImmutableString {
        url.to_string().into()
    }
}
