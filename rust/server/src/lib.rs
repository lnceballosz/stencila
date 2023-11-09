use std::{
    cmp::Ordering,
    fmt::Display,
    net::{IpAddr, SocketAddr},
    path::{Component, PathBuf},
    time::UNIX_EPOCH,
};

use axum::{
    body,
    extract::{Path, State},
    http::{
        header::{ACCEPT_ENCODING, CACHE_CONTROL, CONTENT_ENCODING, CONTENT_TYPE},
        HeaderMap, StatusCode,
    },
    response::{Html, IntoResponse, Response},
    routing::get,
    Router, Server,
};
use format::Format;
use rust_embed::RustEmbed;

use common::{
    clap::{self, Args},
    eyre,
    glob::glob,
    itertools::Itertools,
    tokio::fs::read,
    tracing,
};

/// The current version of Stencila
///
/// Used to improving browser caching of assets by
/// serving static files using versioned paths.
const STENCILA_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The encodings to use when serving static files
///
/// In development do not serve Brotli or Gzip because `make -C web watch` does not
/// build those compressed files (only `make -C web build` does).
#[cfg(debug_assertions)]
const STATIC_ENCODINGS: [(&str, &str); 1] = [("", "")];
#[cfg(not(debug_assertions))]
const STATIC_ENCODINGS: [(&str, &str); 3] = [("br", ".br"), ("gzip", ".gz"), ("", "")];

/// Embedded static files
///
/// During development these are served directly from the folder
/// but are embedded into the binary on release builds.
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../../web/dist"]
#[exclude = "*.map"]
#[exclude = ".gitignore"]
struct Static;

/// Server state available from all routes
#[derive(Default, Clone)]
struct ServerState {
    // The directory that is being served
    dir: PathBuf,

    // Whether files should be served raw
    raw: bool,
}

/// An internal error
#[derive(Debug)]
struct InternalError;

impl InternalError {
    fn new<T: Display>(error: T) -> Self {
        tracing::trace!("{error}");
        Self
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalError")
    }
}

impl std::error::Error for InternalError {}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    }
}

/// Options for the `serve` function
#[derive(Debug, Args)]
pub struct ServeOptions {
    /// The address to serve on
    ///
    /// Defaults to `127.0.0.1` (localhost), use `0.0.0.0` to listen
    /// on all addresses.
    #[arg(long, short, default_value = "127.0.0.1")]
    address: IpAddr,

    /// The port to serve on
    ///
    /// Defaults to port 9000.
    #[arg(long, short, default_value_t = 9000)]
    port: u16,

    /// The directory to serve
    ///
    /// Defaults to the current working directory
    #[arg(long, short, default_value = ".")]
    dir: PathBuf,

    /// Should files be served raw?
    ///
    /// When a request is made to a path that exists within `dir`,
    /// the file will be served with a `Content-Type` header corresponding to
    /// the file's extension.
    #[arg(long, short)]
    raw: bool,
}

/// Start the server
pub async fn serve(
    ServeOptions {
        address,
        port,
        dir,
        raw,
    }: ServeOptions,
) -> eyre::Result<()> {
    let address = SocketAddr::new(address, port);
    tracing::info!("Starting server at http://{address}");

    let router = Router::new()
        .route("/static/*path", get(static_file))
        .route("/*path", get(resolve_path))
        .with_state(ServerState { dir, raw });

    Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

/// Get a static file (e.g. `index.js``)
///
/// Paths to static files include a version so that, in production, the cache control
/// header can be set such that clients should only ever need to make a single request
/// for each version of a static file.
///
/// This cache control is turned off in development so that changes to those files
/// propagate to the browser.
#[tracing::instrument]
async fn home() -> Response {
    let static_version = if cfg!(debug_assertions) {
        "dev"
    } else {
        STENCILA_VERSION
    };

    let page = format!(
        r#"<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8"/>
    <title>Stencila</title>
    <link rel="stylesheet" href="/static/{static_version}/index.css" />
    <script type="module" src="/static/{static_version}/index.js"></script>
</head>
<body>
</body>
</html>"#
    );

    Html(page).into_response()
}

/// Get a static file (e.g. `index.js``)
///
/// Paths to static files include a version so that, in production, the cache control
/// header can be set such that clients should only ever need to make a single request
/// for each version of a static file.
///
/// This cache control is turned off in development so that changes to those files
/// propagate to the browser.
#[tracing::instrument]
async fn static_file(
    Path(path): Path<String>,
    headers: HeaderMap,
) -> Result<Response, InternalError> {
    let path = path.split_once('/').map(|(version, rest)| {
        if version != "dev" && version != STENCILA_VERSION {
            tracing::warn!("Request was made for a different version (current {STENCILA_VERSION}) of a static file: {path}")
        }
        rest.to_string()
    }).unwrap_or(path);

    let accept_encoding = headers
        .get(ACCEPT_ENCODING)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    for (encoding, ext) in STATIC_ENCODINGS {
        if accept_encoding.contains(encoding) {
            let asset_path = [&path, ext].concat();
            if let Some(file) = Static::get(&asset_path) {
                let content_type = mime_guess::from_path(path).first_or_octet_stream();

                let mut response =
                    Response::builder().header(CONTENT_TYPE, content_type.essence_str());

                if !encoding.is_empty() {
                    response = response.header(CONTENT_ENCODING, encoding);
                }

                if !cfg!(debug_assertions) {
                    response = response.header(CACHE_CONTROL, "max-age=31536000, immutable");
                }

                return response
                    .body(body::boxed(body::Full::from(file.data)))
                    .map_err(InternalError::new);
            }
        }
    }

    Ok(StatusCode::NOT_FOUND.into_response())
}

/// Resolve a path into a response
///
/// The path is resolved in response in the following ways:
///
/// - if the path
#[tracing::instrument]
async fn resolve_path(
    State(ServerState { dir, raw, .. }): State<ServerState>,
    Path(path): Path<String>,
) -> Result<Response, InternalError> {
    let path = dir.join(path);

    // Check for a directory traversal attack: if any of the path components include
    // a parent directory (i.e. `/../`) then return a 403.
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Ok((
            StatusCode::FORBIDDEN,
            "Directory traversal is not permitted",
        )
            .into_response());
    }

    // Serve the raw file if flag is on and file exists
    if raw && path.exists() {
        let bytes = read(&path).await.map_err(InternalError::new)?;
        let content_type = mime_guess::from_path(path).first_or_octet_stream();

        return Response::builder()
            .header(CONTENT_TYPE, content_type.essence_str())
            .body(body::boxed(body::Full::from(bytes)))
            .map_err(InternalError::new);
    }

    // If any files have the same stem as the path (everything minus the extension)
    // then use the one with the format with highest preference and latest modification date
    let pattern = format!("{}.*", path.display());
    if let Some(_path) = glob(&pattern)
        .map_err(InternalError::new)?
        .flatten()
        .sorted_by(|a, b| {
            let a_format = Format::from_path(&a).unwrap_or_default();
            let b_format = Format::from_path(&b).unwrap_or_default();
            match a_format.rank().cmp(&b_format.rank()) {
                Ordering::Equal => {
                    let a_modified = std::fs::metadata(&a)
                        .and_then(|metadata| metadata.modified())
                        .unwrap_or(UNIX_EPOCH);
                    let b_modified = std::fs::metadata(&b)
                        .and_then(|metadata| metadata.modified())
                        .unwrap_or(UNIX_EPOCH);
                    a_modified.cmp(&b_modified).reverse()
                }
                ordering => ordering,
            }
        })
        .next()
    {
        return Ok(StatusCode::OK.into_response());
    }

    Ok(StatusCode::NOT_FOUND.into_response())
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;
    use common::{eyre::Result, tokio};

    use super::*;

    /// Test the `resolve_path` method using the `routing` example
    #[tokio::test]
    async fn test_resolve_path() -> Result<()> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/routing")
            .canonicalize()?;

        // Will forbid paths with `..` in them
        for path in [
            "..",
            "../..",
            "some/..",
            "../some",
            "some/../some",
            "some/../..",
        ] {
            let response =
                resolve_path(State(ServerState::default()), Path(path.to_string())).await?;
            assert_eq!(response.status(), StatusCode::FORBIDDEN);
        }

        // With `raw` flag will serve static content
        for (path, mime) in [
            ("README.md", "text/markdown"),
            ("bird/owl/README.md", "text/markdown"),
        ] {
            let response = resolve_path(
                State(ServerState {
                    dir: dir.clone(),
                    raw: true,
                    ..Default::default()
                }),
                Path(path.to_string()),
            )
            .await?;
            assert_eq!(
                response.headers().get("content-type"),
                Some(&HeaderValue::from_static(mime))
            );
        }

        Ok(())
    }
}
