//! Helper e macro per la registrazione di schemi OpenAPI.
//!
//! Questo modulo fornisce strumenti per registrare automaticamente i tipi
//! come schemi nella documentazione OpenAPI.

use utoipa::ToSchema;
use utoipa::openapi::Components;

/// Macro per registrare uno schema in OpenAPI Components.
///
/// Questa macro semplifica la registrazione di un tipo come schema OpenAPI,
/// estraendo automaticamente il nome e lo schema tramite i trait `ToSchema`
/// e `PartialSchema` di utoipa.
///
/// # Arguments
///
/// * `$components` - Mut reference ai componenti OpenAPI
/// * `$schema_type` - Il tipo da registrare (deve implementare `ToSchema`)
///
/// # Esempio
///
/// ```rust,no_run
/// use eywa_utoipa::register_schema;
/// use utoipa::openapi::Components;
///
/// #[derive(utoipa::ToSchema)]
/// struct MyResponse {
///     message: String,
/// }
///
/// fn register_schemas(components: &mut Components) {
///     register_schema!(components, MyResponse);
/// }
/// ```
#[macro_export]
macro_rules! register_schema {
    ($components:expr, $schema_type:ty) => {
        {
            use utoipa::{ToSchema, PartialSchema};
            let name = <$schema_type as ToSchema>::name().to_string();
            let schema = <$schema_type as PartialSchema>::schema();
            $components.schemas.insert(name, schema);
        }
    };
}

pub use register_schema;

/// Helper per registrare una lista di tipi come schemi.
///
/// Questa funzione può essere utilizzata dalle macro per registrare
/// più schemi in una sola volta.
///
/// # Type Parameters
///
/// * `T` - Il tipo da registrare (deve implementare `ToSchema`)
///
/// # Arguments
///
/// * `components` - Mut reference ai componenti OpenAPI
///
/// # Esempio
///
/// ```rust,no_run
/// use eywa_utoipa::register_schema_list;
/// use utoipa::openapi::Components;
///
/// fn register_all(components: &mut Components) {
///     register_schema_list::<MyResponse1>(components);
///     register_schema_list::<MyResponse2>(components);
/// }
pub fn register_schema_list<T: ToSchema + 'static>(components: &mut Components) {
    let name = T::name().to_string();
    let schema = T::schema();
    components.schemas.insert(name, schema);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize, ToSchema)]
    struct TestSchema {
        name: String,
        value: i32,
    }

    #[test]
    fn test_register_schema_macro() {
        let mut components = Components::new();
        register_schema!(&mut components, TestSchema);

        assert!(components.schemas.contains_key("TestSchema"));
    }

    #[test]
    fn test_register_schema_list() {
        let mut components = Components::new();
        register_schema_list::<TestSchema>(&mut components);

        assert!(components.schemas.contains_key("TestSchema"));
    }
}
