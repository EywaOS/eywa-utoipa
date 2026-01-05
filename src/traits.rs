//! Trait per la registrazione di schemi e percorsi OpenAPI.
//!
//! Questi trait sono utilizzati da `eywa-axum-macros` per generare
//! automaticamente la documentazione OpenAPI dai controller.

use utoipa::openapi::{Components, OpenApi};
use axum::Router;

/// Trait per registrare schemi e percorsi nella documentazione OpenAPI.
///
/// Questo trait viene implementato automaticamente dalla macro `#[controller]`
/// e definisce come un controller contribuisce alla documentazione OpenAPI.
///
/// # Esempio
///
/// ```rust,no_run
/// use eywa_utoipa::OpenApiRegistrar;
/// use utoipa::openapi::Components;
///
/// struct MyController;
///
/// impl OpenApiRegistrar for MyController {
///     fn register_schemas(components: &mut Components) {
///         // Registra gli schemi del controller
///     }
///
///     fn prefix() -> String {
///         "/v1/my_controller".to_string()
///     }
///
///     fn tag() -> String {
///         "My Controller".to_string()
///     }
/// }
/// ```
pub trait OpenApiRegistrar {
    /// Registra gli schemi (tipi) nel componente OpenAPI.
    ///
    /// Questo metodo viene chiamato durante la costruzione della documentazione
    /// per aggiungere tutti gli schemi definiti nel controller.
    ///
    /// # Default
    ///
    /// Di default non fa nulla.
    fn register_schemas(components: &mut Components) {
        let _ = components;
    }

    /// Registra i percorsi (endpoint) nella documentazione OpenAPI.
    ///
    /// Questo metodo viene chiamato per aggiungere tutti i path definiti
    /// nel controller alla documentazione.
    ///
    /// # Default
    ///
    /// Di default non fa nulla.
    fn register_paths(openapi: &mut OpenApi) {
        let _ = openapi;
    }

    /// Restituisce il prefisso del controller (es. "/v1/projects").
    ///
    /// Questo prefisso viene utilizzato per montare il router del controller
    /// e come base per i percorsi OpenAPI.
    ///
    /// # Default
    ///
    /// Restituisce una stringa vuota.
    fn prefix() -> String {
        String::new()
    }

    /// Restituisce il tag del controller per la documentazione OpenAPI (es. "Projects").
    ///
    /// I tag vengono utilizzati per raggruppare gli endpoint nella documentazione.
    ///
    /// # Default
    ///
    /// Estrae il nome del tipo dal `type_name`.
    fn tag() -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("Unknown")
            .to_string()
    }
}

/// Trait per convertire un controller in un Axum Router.
///
/// Questo trait combina la funzionalità di routing di Axum con la registrazione
/// OpenAPI, permettendo ai controller di essere sia router che contributori
/// alla documentazione.
///
/// # Type Parameters
///
/// * `S` - Il tipo di stato (State) che verrà passato agli handler
///
/// # Esempio
///
/// ```rust,no_run
/// use eywa_utoipa::IntoRouter;
/// use axum::Router;
/// use std::sync::Arc;
///
/// struct AppState { /* ... */ }
///
/// struct MyController;
///
/// impl IntoRouter<Arc<AppState>> for MyController {
///     fn into_router(state: Arc<AppState>) -> Router {
///         Router::new().route("/", axum::routing::get(handler))
///             .with_state(state)
///     }
/// }
/// ```
pub trait IntoRouter<S>: OpenApiRegistrar {
    /// Converte il controller in un Axum Router.
    ///
    /// Questo metodo deve restituire un router configurato con tutti gli
    /// endpoint del controller.
    fn into_router(state: S) -> Router;

    /// Restituisce una lista di route OpenAPI registrate dal controller.
    ///
    /// Questo metodo viene utilizzato per tracciare quali percorsi sono stati
    /// aggiunti alla documentazione.
    ///
    /// # Default
    ///
    /// Restituisce un `Vec` vuoto.
    fn openapi_routes() -> Vec<String> {
        Vec::new()
    }
}
