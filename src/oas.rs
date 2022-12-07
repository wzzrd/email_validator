use log::error;
use paperclip::v2::models::{
    Api, Contact, DefaultApiRaw, DefaultParameterRaw, DefaultResponseRaw, DefaultSchemaRaw,
    ExternalDocs, Info, License, OperationProtocol, Tag,
};
use std::collections::{BTreeMap, BTreeSet};

pub fn build_spec(
    version: &str,
    gateway: &str,
) -> Api<DefaultParameterRaw, DefaultResponseRaw, DefaultSchemaRaw> {
    log::info!("Setting schema defaults");
    let mut spec = DefaultApiRaw::default();
    let badges = serde_json::json!(
        [
            {
                "name": "env",
                "value": "dev"
            },
            {
                "name": "security",
                "value": "medium"
            },
            {
                "name": "region",
                "value": "global"
            }
        ]
    );
    // We need this to be a <String, Value> because below, we are inserting Strings, Bools and Vecs
    // into that Value.
    let mut info_exts = BTreeMap::new();
    info_exts.insert("x-category".to_string(), serde_json::json!("Utility APIs"));
    info_exts.insert(
        "x-long-description".to_string(),
        serde_json::Value::String(
            "Use this API to syntactically validate email address".to_string(),
        ),
    );
    info_exts.insert(
        "x-thumbnail".to_string(),
        serde_json::json!(
            "https://en.gravatar.com/userimage/3149428/abb6f0635c488a6833a4966c9cff4ea2.jpeg"
        ),
    );
    info_exts.insert(
        "x-version-lifecycle".to_string(),
        serde_json::json!("active"),
    );
    info_exts.insert(
        "x-collections".to_string(),
        serde_json::json!(["consumer-onboarding"]),
    );
    info_exts.insert(
        "x-website".to_string(),
        serde_json::json!("https://100things.wzzrd.com"),
    );
    info_exts.insert("x-public".to_string(), serde_json::Value::Bool(false));
    info_exts.insert(
        "termsOfService".to_string(),
        serde_json::json!("https:///www.wzzrd.com/tos"),
    );
    info_exts.insert("x-badges".to_string(), badges);

    let mut root_exts = BTreeMap::new();
    root_exts.insert(
        "x-gateways".to_string(),
        serde_json::json!([{ "url": String::from(gateway) }]),
    );
    root_exts.insert(
        "x-documentation".to_string(),
        serde_json::json!(
            {
                "readme": include_str!("../README.md").to_string(),
                "spotlights":
                [
                    {
                        "title": "paperclip-rs",
                        "description": "The Rust crate this API is built on",
                        "link": "https://github.com/reacherhq/check-if-email-exists"
                    },
                    {
                        "title": "GitHub repo",
                        "description": "The GitHub repo for this API",
                        "link": "https://github.com/wzzrd/email_validator"
                    }
                ],
                "tutorials":
                [
                    {
                        "title": "How to call this API",
                        "description": include_str!("../HOW_TO_CALL.md").to_string(),
                        "link": "https://images.pexels.com/photos/965345/pexels-photo-965345.jpeg"
                    },
                    {
                        "title": "Our roadmap",
                        "description": include_str!("../ROADMAP.md").to_string(),
                        "link": "https://img.freepik.com/free-vector/gradient-roadmap-infographic-template_23-2149014238.jpg"
                    }
                ]
            }
        ),
    );

    // let mut tags = BTreeMap::new();
    spec.tags = vec![Tag {
        name: "v1".to_string(),
        description: Some("Version 1 endpoints".to_string()),
        external_docs: Some(ExternalDocs {
            url: "https://100things.nontoonyt.com/gogs/wzzrd/email_validator".to_string(),
            description: Some("Upstream repo".to_string()),
        }),
    }];

    spec.extensions = root_exts;
    spec.schemes = BTreeSet::new();
    spec.schemes.insert(OperationProtocol::Https);

    spec.host = Some(gateway.into());
    spec.base_path = Some("/".to_string());

    spec.info = Info {
        version: version.into(),
        contact: Some(Contact {
            name: Some("Maxim Burgerhout".to_string()),
            email: Some("maxim@wzzrd.com".to_string()),
            url: Some("https://www.wzzrd.com".to_string()),
        }),
        license: Some(License {
            name: Some("something legal".to_string()),
            url: Some("https://www.wzzrd.com".to_string()),
        }),
        title: "Email address verification".into(),
        description: Some("This API verifies the validity of email addresses".to_string()),
        extensions: info_exts,
    };
    spec
}
