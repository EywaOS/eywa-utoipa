//! Helper per la registrazione di percorsi OpenAPI.
//!
//! Questo modulo fornisce funzioni per registrare i percorsi (endpoint)
//! nella documentazione OpenAPI, utilizzando le struct generate da utoipa.

use utoipa::openapi::{OpenApi, PathItem};

/// Registra un path utoipa nella documentazione OpenAPI.
///
/// Questa funzione aggiunge un `PathItem` alla documentazione OpenAPI,
/// utilizzando il percorso specificato.
///
/// # Arguments
///
/// * `openapi` - Mut reference alla struttura OpenAPI
/// * `path` - Il percorso dell'endpoint (es. "/v1/projects")
/// * `path_item` - Il PathItem contenente le operazioni HTTP
///
/// # Esempio
///
/// ```rust,no_run
/// use eywa_utoipa::register_utoipa_path;
/// use utoipa::openapi::{OpenApi, PathItem};
///
/// fn register_paths(openapi: &mut OpenApi) {
///     let path_item = PathItem::new();
///     register_utoipa_path(openapi, "/v1/projects", path_item);
/// }
/// ```
pub fn register_utoipa_path(openapi: &mut OpenApi, path: &str, path_item: PathItem) {
    openapi.paths.paths.insert(path.to_string(), path_item);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_utoipa_path() {
        let mut openapi = OpenApi::new(
            utoipa::openapi::Info::new("Test", "1.0.0"),
            utoipa::openapi::Paths::new(),
        );

        let path_item = PathItem::default();
        register_utoipa_path(&mut openapi, "/test", path_item);

        assert!(openapi.paths.paths.contains_key("/test"));
    }
}
