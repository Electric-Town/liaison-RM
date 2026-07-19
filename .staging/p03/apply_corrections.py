#!/usr/bin/env python3
"""Apply reviewed corrections to the P03 source candidate before validation."""

from pathlib import Path
import re


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"expected one {label} replacement, found {count}")
    return text.replace(old, new, 1)


cargo = Path("adapters/vault-markdown/Cargo.toml")
text = cargo.read_text(encoding="utf-8")
if "chrono.workspace = true" not in text.split("[dev-dependencies]", 1)[0]:
    text = replace_once(
        text,
        "[dependencies]\ncap-fs-ext.workspace = true\n",
        "[dependencies]\ncap-fs-ext.workspace = true\nchrono.workspace = true\n",
        "normal chrono dependency",
    )
text = text.replace("\n[dev-dependencies]\nchrono.workspace = true\n", "\n[dev-dependencies]\n", 1)
cargo.write_text(text, encoding="utf-8")

operations = Path("adapters/vault-markdown/src/operations.rs")
text = operations.read_text(encoding="utf-8")
text = replace_once(
    text,
    "        let bytes = read_file(&staged, &staged_name(target.ordinal)).map_err(|error| {",
    "        let staged_filename = staged_name(target.ordinal);\n"
    "        let bytes = read_file(&staged, Path::new(&staged_filename)).map_err(|error| {",
    "staged target path",
)
text = replace_once(
    text,
    '            map_io("read staged target", error)\n',
    "            error\n",
    "staged target error mapping",
)
text = replace_once(
    text,
    "pub(crate) fn execute_with_fault(\n",
    "// The durability protocol stays linear so flush and decision ordering remains auditable.\n"
    "#[allow(clippy::too_many_lines)]\n"
    "pub(crate) fn execute_with_fault(\n",
    "durability protocol lint rationale",
)
text = replace_once(
    text,
    "fn map_io(action: &str, error: io::Error) -> RecoverableOperationError {",
    "fn map_io(action: &str, error: &io::Error) -> RecoverableOperationError {",
    "borrowed IO error",
)
text, borrow_count = re.subn(
    r'map_io\(("[^"]+"), (error|sync_error)\)',
    r'map_io(\1, &\2)',
    text,
)
if borrow_count == 0:
    raise SystemExit("expected at least one borrowed map_io call")
operations.write_text(text, encoding="utf-8")

vault = Path("adapters/vault-markdown/src/lib.rs")
text = vault.read_text(encoding="utf-8")

current_pattern = re.compile(
    r'(?m)^(?P<i>[ \t]*)let current = bound_find_person\(root, person\.id\)\n'
    r'(?P=i)    \.ok\(\)\n'
    r'(?P=i)    \.map\(\|\(_, parsed\)\| parsed\.document\.revision\.get\(\)\)\n'
    r'(?P=i)    \.unwrap_or\(expected_revision\.get\(\)\);'
)


def replace_current(match: re.Match[str]) -> str:
    indent = match.group("i")
    return (
        f"{indent}let current = bound_find_person(root, person.id)\n"
        f"{indent}    .ok()\n"
        f"{indent}    .map_or(expected_revision.get(), |(_, parsed)| {{\n"
        f"{indent}        parsed.document.revision.get()\n"
        f"{indent}    }});"
    )


text, current_count = current_pattern.subn(replace_current, text, count=1)
if current_count != 1:
    raise SystemExit(f"expected one revision fallback replacement, found {current_count}")

save_pattern = re.compile(
    r'(?m)^(?P<i>[ \t]*)let expected = person\.revision;\n'
    r'(?P=i)assert!\(person\.rename\("Alex M\. Murphy"\)\.is_ok\(\)\);\n'
    r'(?P=i)assert!\(bound\.save\(&person, expected\)\.is_ok\(\)\);'
)


def replace_save(match: re.Match[str]) -> str:
    indent = match.group("i")
    return (
        f"{indent}let expected = person.revision;\n"
        f"{indent}assert!(person.rename(\"Alex M. Murphy\").is_ok());\n"
        f"{indent}let save_repository =\n"
        f"{indent}    people_mutation_repository(&work, operation_context());\n"
        f"{indent}assert!(save_repository.save(&person, expected).is_ok());"
    )


text, save_count = save_pattern.subn(replace_save, text, count=1)
if save_count != 1:
    raise SystemExit(f"expected one save-context replacement, found {save_count}")

order_pattern = re.compile(
    r'(?ms)^(?P<i>[ \t]*)let bound = people_mutation_repository\(&work, operation_context\(\)\);\n'
    r'(?P=i)let create = CreatePerson::new\(&bound\);\n'
    r'(?P=i)assert!\(\s*create\s*\.execute\(PersonId::new\(\), "Zara Example", None\)\s*\.is_ok\(\)\s*\);\n'
    r'(?P=i)assert!\(\s*create\s*\.execute\(PersonId::new\(\), "Alex Example", None\)\s*\.is_ok\(\)\s*\);\n'
    r'(?P=i)let people = ListPeople::new\(&bound\)\.execute\(false\);'
)


def replace_order(match: re.Match[str]) -> str:
    indent = match.group("i")
    return (
        f"{indent}let zara_repository = people_mutation_repository(&work, operation_context());\n"
        f"{indent}assert!(\n"
        f"{indent}    CreatePerson::new(&zara_repository)\n"
        f"{indent}        .execute(PersonId::new(), \"Zara Example\", None)\n"
        f"{indent}        .is_ok()\n"
        f"{indent});\n"
        f"{indent}let alex_repository = people_mutation_repository(&work, operation_context());\n"
        f"{indent}assert!(\n"
        f"{indent}    CreatePerson::new(&alex_repository)\n"
        f"{indent}        .execute(PersonId::new(), \"Alex Example\", None)\n"
        f"{indent}        .is_ok()\n"
        f"{indent});\n"
        f"{indent}let people = ListPeople::new(&alex_repository).execute(false);"
    )


text, order_count = order_pattern.subn(replace_order, text, count=1)
if order_count != 1:
    raise SystemExit(f"expected one stable-order context replacement, found {order_count}")

vault.write_text(text, encoding="utf-8")
print("P03 reviewed corrections applied")
