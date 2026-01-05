//! Prelude per eywa-utoipa.
//!
//! Questo modulo re-exporta tutti i tipi e trait più comuni per un import facile.
//!
//! # Esempio
//!
//! ```rust,no_run
//! use eywa_utoipa::prelude::*;
//!
//! #[derive(ToSchema)]
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

// Re-export da utoipa
pub use utoipa::{self, OpenApi};

// Re-export da utoipa-axum
pub use utoipa_axum::router::OpenApiRouter;

// Re-export da eywa-utoipa
pub use crate::{
    OpenApiRegistrar,
    IntoRouter,
    OpenApiBuilder,
    HateoasSchema,
    register_utoipa_path,
};

// Re-export da eywa-hateoas (comunemente usati insieme)
pub use eywa_hateoas::{
    HateoasResponse,
    Link,
    Links,
    CollectionResponse,
    CollectionMeta,
};
