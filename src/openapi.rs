//! Builder per la costruzione della documentazione OpenAPI.
//!
//! Questo modulo fornisce un builder pattern per costruire progressivamente
//! la documentazione OpenAPI, aggiungendo info, tag, schemi di sicurezza
//! e registrando schemi e percorsi.

use utoipa::openapi::{
    OpenApi, Info, Components, Paths, Tag,
};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use std::collections::HashMap;

/// Builder per costruire la documentazione OpenAPI.
///
/// Questo builder permette di costruire progressivamente una struttura
/// OpenAPI completa, aggiungendo info, tag, schemi di sicurezza e
/// registrando schemi e percorsi tramite callback.
///
/// # Esempio
///
/// ```rust,no_run
/// use eywa_utoipa::OpenApiBuilder;
/// use utoipa::openapi::Info;
///
/// let openapi = OpenApiBuilder::new()
///     .with_info(Info::new("My API", "1.0.0"))
///     .with_bearer_security("bearer")
///     .register_schemas(|components| {
///         // Registra gli schemi
///     })
///     .register_paths(|openapi| {
///         // Registra i percorsi
///     })
///     .build();
/// ```
pub struct OpenApiBuilder {
    info: Option<Info>,
    tags: Vec<Tag>,
    security_schemes: HashMap<String, SecurityScheme>,
    schema_fns: Vec<Box<dyn FnOnce(&mut Components)>>,
    path_fns: Vec<Box<dyn FnOnce(&mut OpenApi)>>,
}

impl OpenApiBuilder {
    /// Crea un nuovo builder OpenAPI vuoto.
    pub fn new() -> Self {
        Self {
            info: None,
            tags: Vec::new(),
            security_schemes: HashMap::new(),
            schema_fns: Vec::new(),
            path_fns: Vec::new(),
        }
    }

    /// Imposta le informazioni della API.
    ///
    /// # Arguments
    ///
    /// * `info` - La struct `Info` con titolo, versione, ecc.
    pub fn with_info(mut self, info: Info) -> Self {
        self.info = Some(info);
        self
    }

    /// Imposta i tag della documentazione.
    ///
    /// # Arguments
    ///
    /// * `tags` - Vettore di tag per raggruppare gli endpoint
    pub fn with_tags(mut self, tags: Vec<Tag>) -> Self {
        self.tags = tags;
        self
    }

    /// Aggiunge un singolo tag alla documentazione.
    ///
    /// # Arguments
    ///
    /// * `tag` - Il tag da aggiungere
    pub fn add_tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }

    /// Aggiunge uno schema di sicurezza personalizzato.
    ///
    /// # Arguments
    ///
    /// * `name` - Nome dello schema di sicurezza
    /// * `scheme` - Lo schema di sicurezza (Bearer, API Key, ecc.)
    pub fn with_security(mut self, name: &str, scheme: SecurityScheme) -> Self {
        self.security_schemes.insert(name.to_string(), scheme);
        self
    }

    /// Aggiunge uno schema di sicurezza Bearer JWT.
    ///
    /// Questo è un metodo helper che crea automaticamente uno schema
    /// di sicurezza HTTP Bearer con formato JWT.
    ///
    /// # Arguments
    ///
    /// * `name` - Nome dello schema (tipicamente "bearer" o "bearer_auth")
    pub fn with_bearer_security(mut self, name: &str) -> Self {
        let scheme = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );
        self.security_schemes.insert(name.to_string(), scheme);
        self
    }

    /// Registra una funzione che verrà chiamata per aggiungere schemi.
    ///
    /// # Arguments
    ///
    /// * `f` - Funzione che prende un mut reference ai Components e registra gli schemi
    pub fn register_schemas<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Components) + 'static,
    {
        self.schema_fns.push(Box::new(f));
        self
    }

    /// Registra una funzione che verrà chiamata per aggiungere percorsi.
    ///
    /// # Arguments
    ///
    /// * `f` - Funzione che prende un mut reference a OpenAPI e registra i percorsi
    pub fn register_paths<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut OpenApi) + 'static,
    {
        self.path_fns.push(Box::new(f));
        self
    }

    /// Costruisce la struttura OpenAPI finale.
    ///
    /// Questo metodo:
    /// 1. Crea la struttura OpenAPI base con Info (o default)
    /// 2. Aggiunge i tag se presenti
    /// 3. Crea i Components e aggiunge gli schemi di sicurezza
    /// 4. Esegue tutte le funzioni di registrazione schemi
    /// 5. Aggiunge i Components alla struttura se non vuoti
    /// 6. Esegue tutte le funzioni di registrazione percorsi
    pub fn build(mut self) -> OpenApi {
        let info = self.info.unwrap_or_else(|| {
            Info::new("API", "1.0.0")
        });

        let mut openapi = OpenApi::new(info, Paths::new());

        // Add tags if present
        if !self.tags.is_empty() {
            openapi.tags = Some(self.tags);
        }

        let mut components = Components::new();

        for (name, scheme) in self.security_schemes {
            components.add_security_scheme(&name, scheme);
        }

        for schema_fn in self.schema_fns {
            schema_fn(&mut components);
        }

        if !components.schemas.is_empty() || !components.security_schemes.is_empty() {
            openapi.components = Some(components);
        }

        for path_fn in self.path_fns {
            path_fn(&mut openapi);
        }

        openapi
    }
}

impl Default for OpenApiBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_builder_basic() {
        let info = Info::new("Test API", "1.0.0");
        let openapi = OpenApiBuilder::new()
            .with_info(info)
            .build();

        assert_eq!(openapi.info.title, "Test API");
        assert_eq!(openapi.info.version, "1.0.0");
    }

    #[test]
    fn test_openapi_builder_with_tags() {
        let tag = Tag::new("Test Tag");
        let openapi = OpenApiBuilder::new()
            .add_tag(tag)
            .build();

        assert!(openapi.tags.is_some());
        assert_eq!(openapi.tags.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_openapi_builder_with_bearer_security() {
        let openapi = OpenApiBuilder::new()
            .with_bearer_security("bearer")
            .build();

        assert!(openapi.components.is_some());
        let components = openapi.components.as_ref().unwrap();
        assert!(components.security_schemes.contains_key("bearer"));
    }

    #[test]
    fn test_openapi_builder_register_schemas() {
        use utoipa::ToSchema;
        use serde::Serialize;

        #[derive(Serialize, ToSchema)]
        struct TestSchema {
            name: String,
        }

        let openapi = OpenApiBuilder::new()
            .register_schemas(|components| {
                register_schema!(components, TestSchema);
            })
            .build();

        assert!(openapi.components.is_some());
        let components = openapi.components.as_ref().unwrap();
        assert!(components.schemas.contains_key("TestSchema"));
    }

    #[test]
    fn test_openapi_builder_register_paths() {
        let openapi = OpenApiBuilder::new()
            .register_paths(|openapi| {
                let path_item = utoipa::openapi::PathItem::default();
                register_utoipa_path(openapi, "/test", path_item);
            })
            .build();

        assert!(openapi.paths.paths.contains_key("/test"));
    }
}
