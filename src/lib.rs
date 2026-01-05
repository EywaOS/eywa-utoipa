//! # Eywa Utoipa
//!
//! Integrazione tra [Utoipa](https://github.com/juhaku/utoipa) e l'ecosistema Eywa.
//!
//! Questa crate fornisce trait, helper e builder per la generazione automatica
//! di documentazione OpenAPI, seguendo il pattern di `eywa-hateoas`.
//!
//! ## Architettura
//!
//! - **Nessuna macro procedurale**: Le macro (`#[controller]`, `#[route]`, `links(...)`)
//!   rimangono in `eywa-axum-macros`
//! - **Solo trait e helper**: Fornisce i "mattoni" che le macro usano per generare codice
//! - **Integrazione HATEOAS**: Si connette con `eywa-hateoas` per documentazione completa
//!
//! ## Esempio
//!
//! ```rust,no_run
//! use eywa_utoipa::prelude::*;
//! use serde::Serialize;
//!
//! #[derive(Serialize, ToSchema)]
//! struct MyResponse {
//!     message: String,
//! }
//!
//! impl OpenApiRegistrar for MyController {
//!     fn register_schemas(components: &mut Components) {
//!         register_schema!(components, MyResponse);
//!     }
//! }
//! ```

mod traits;
mod schema;
mod path;
mod openapi;
mod hateoas;
mod prelude;

// Re-export utoipa types
pub use utoipa::{
    self,
    IntoParams,
    OpenApi,
    ToSchema,
    Path,
    openapi::OpenApi as OpenApiStruct,
};
pub use utoipa_axum::router::OpenApiRouter;

// Re-export moduli interni
pub use traits::{OpenApiRegistrar, IntoRouter};
pub use path::register_utoipa_path;
pub use openapi::OpenApiBuilder;
pub use hateoas::HateoasSchema;

// Prelude
pub use prelude::*;
