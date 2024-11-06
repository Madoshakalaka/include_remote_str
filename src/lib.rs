use std::path::PathBuf;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use once_cell::sync::Lazy;
use reqwest_middleware::ClientWithMiddleware;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

use serde::Deserialize;
use std::process::Command;

// If we are in a workspace, lookup `workspace_root` since `CARGO_MANIFEST_DIR` won't
// reflect the workspace dir: https://github.com/rust-lang/cargo/issues/3946
static CLIENT: Lazy<ClientWithMiddleware> = Lazy::new(|| {
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
        .expect("`CARGO_MANIFEST_DIR` must be set")
        .into();

    let cargo = std::env::var("CARGO").expect("`CARGO` must be set");

    let output = Command::new(cargo)
        .args(["metadata", "--format-version=1", "--no-deps"])
        .current_dir(&manifest_dir)
        .env_remove("__CARGO_FIX_PLZ")
        .output()
        .expect("Could not fetch metadata");

    #[derive(Deserialize)]
    struct CargoMetadata {
        workspace_root: PathBuf,
    }

    let metadata: CargoMetadata =
        serde_json::from_slice(&output.stdout).expect("Invalid `cargo metadata` output");

    ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager {
                path: metadata.workspace_root.join("include-remote-str-cache"),
            },
            options: HttpCacheOptions::default(),
        }))
        .build()
});

#[proc_macro]
pub fn include_remote_str(url: TokenStream) -> TokenStream {
    let url = syn::parse_macro_input!(url as syn::LitStr);

    let url = url.value();

    let text = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { CLIENT.get(url).send().await.unwrap().text().await.unwrap() });

    let text = syn::LitStr::new(&text, Span::call_site());

    quote! {
        #text
    }
    .into()
}
