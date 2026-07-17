use std::{fs, path::PathBuf};

#[test]
fn provider_wit_contract_contains_the_versioned_object_store_surface() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../interfaces/wit/liaison-provider.wit");
    let content = fs::read_to_string(path);
    assert!(content.is_ok(), "provider WIT must be readable");
    let Ok(content) = content else {
        return;
    };

    let required_fragments = [
        "package electric-town:liaison-provider@0.1.0;",
        "interface object-store",
        "put-immutable",
        "get:",
        "head:",
        "list:",
        "delete-if-permitted",
        "replace-manifest-if-revision",
        "world provider",
        "export object-store;",
    ];
    for fragment in required_fragments {
        assert!(
            content.contains(fragment),
            "provider WIT is missing required fragment: {fragment}"
        );
    }

    let opening = content.chars().filter(|value| *value == '{').count();
    let closing = content.chars().filter(|value| *value == '}').count();
    assert_eq!(opening, closing, "provider WIT braces must be balanced");
}
