use rhai::def_package;
use rhai::plugin::*;

pub(crate) mod url;

def_package! {
    pub UrlPackage(lib) {
       combine_with_exported_module!(lib, "rhai_url", url::url_module);
    }
}
