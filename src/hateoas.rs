//! Integrazione con [eywa-hateoas](../eywa-hateoas) per OpenAPI.
//!
//! Questo modulo fornisce helper per registrare automaticamente tutti gli
//! schemi HATEOAS nella documentazione OpenAPI.

use eywa_hateoas::{HateoasResponse, Link, Links, CollectionResponse, CollectionMeta};
use utoipa::openapi::Components;
use crate::register_schema;

/// Helper per registrare tutti gli schemi HATEOAS in OpenAPI.
///
/// Questa struct fornisce metodi statici per registrare tutti i tipi
/// definiti in `eywa-hateoas` come schemi OpenAPI.
///
/// # Esempio
///
/// ```rust,no_run
/// use eywa_utoipa::HateoasSchema;
/// use utoipa::openapi::Components;
///
/// fn register_all_schemas(components: &mut Components) {
///     HateoasSchema::register_all(components);
/// }
/// ```
pub struct HateoasSchema;

impl HateoasSchema {
    /// Registra tutti gli schemi HATEOAS in OpenAPI Components.
    ///
    /// Questo metodo registra:
    /// - `HateoasResponse<T>` (come generico con serde_json::Value)
    /// - `Link`
    /// - `Links` (HashMap<String, Link>)
    /// - `CollectionResponse<T>` (come generico con serde_json::Value)
    /// - `CollectionMeta`
    ///
    /// # Arguments
    ///
    /// * `components` - Mut reference ai componenti OpenAPI
    ///
    /// # Esempio
    ///
    /// ```rust,no_run
    /// use eywa_utoipa::HateoasSchema;
    /// use utoipa::openapi::Components;
    ///
    /// let mut components = Components::new();
    /// HateoasSchema::register_all(&mut components);
    /// ```
    pub fn register_all(components: &mut Components) {
        // HateoasResponse<T> - usiamo serde_json::Value come placeholder per il generico T
        register_schema!(components, HateoasResponse<serde_json::Value>);

        // Link
        register_schema!(components, Link);

        // Links (HashMap<String, Link>)
        register_schema!(components, Links);

        // CollectionResponse<T>
        register_schema!(components, CollectionResponse<serde_json::Value>);

        // CollectionMeta
        register_schema!(components, CollectionMeta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hateoas_schema_register_all() {
        let mut components = Components::new();
        HateoasSchema::register_all(&mut components);

        // Verifica che tutti gli schemi siano registrati
        assert!(components.schemas.contains_key("HateoasResponse_Value"));
        assert!(components.schemas.contains_key("Link"));
        assert!(components.schemas.contains_key("Links"));
        assert!(components.schemas.contains_key("CollectionResponse_Value"));
        assert!(components.schemas.contains_key("CollectionMeta"));
    }
}
