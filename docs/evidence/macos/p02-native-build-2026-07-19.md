# P02 universal macOS review build — 2026-07-19

Status: **local uninstalled implementation-head build evidence; not release or installed-app QA**

## Provenance

- source commit: `4acf088b7dc5d791343fcafc1f41db462cb17626`;
- target: `universal-apple-darwin`;
- minimum macOS version: `10.15`;
- bundle identifier: `io.github.electric-town.liaison-rm`;
- bundle version: `0.1.0-alpha.1`;
- local build output: `/private/tmp/liaison-p02-mac-build-4acf088/universal-apple-darwin/release/bundle/macos/Liaison RM.app`.

The source commit contains the P02 Workspace Session implementation and the
review fixes for global native-operation serialisation, replacement-cleanup
recovery, and nonblocking special-file reads. The later evidence-only commit
does not change application source.

## Build and verification

The universal application was built locally with:

```text
CARGO_TARGET_DIR=/private/tmp/liaison-p02-mac-build-4acf088 \
MACOSX_DEPLOYMENT_TARGET=10.15 \
cargo tauri build --target universal-apple-darwin --bundles app
```

Tauri initially emitted an application with a linker/ad-hoc executable code
directory but no valid bundle signature. This command correctly failed:

```text
codesign --verify --deep --strict --verbose=4 "Liaison RM.app"
code object is not signed at all
```

The local review bundle was then ad-hoc signed explicitly:

```text
codesign --force --deep --sign - "Liaison RM.app"
codesign --verify --deep --strict --verbose=4 "Liaison RM.app"
valid on disk
satisfies its Designated Requirement
```

Post-signing inspection reported:

```text
Identifier=io.github.electric-town.liaison-rm
Signature=adhoc
TeamIdentifier=not set
Architectures: x86_64 arm64
Executable SHA-256: cb175446ab8950c878d5aff936b49720efe6c65c7bacbd949e58d7cc0169ea6f
```

## Claim boundary

This record proves a local universal build and strict verification after an
explicit ad-hoc review signature. It does not prove Developer ID signing,
notarisation, Gatekeeper acceptance, clean-machine installation, installed
Workspace/Person/Health behavior, relaunch recovery, native accessibility,
network denial, P03 recovery, B0 acceptance, or supported distribution. The
exact pull-request head must still pass the native macOS and cross-platform
remote workflows.
