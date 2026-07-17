use std::path::PathBuf;

#[test]
fn provider_wit_contract_parses_and_resolves() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../interfaces/wit/liaison-provider.wit");
    let mut resolve = wit_parser::Resolve::default();
    let result = resolve.push_path(path);
    assert!(result.is_ok(), "provider WIT did not resolve: {result:?}");
}
