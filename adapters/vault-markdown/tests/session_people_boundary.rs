#[test]
fn people_repository_requires_a_live_workspace_work_guard() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/unbound-store-is-not-person-repository.rs");
    cases.compile_fail("tests/ui/repository-cannot-escape-work-guard.rs");
    cases.pass("tests/ui/repository-is-available-under-guard.rs");
}
