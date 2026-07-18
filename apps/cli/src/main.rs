use clap::{Args, Parser, Subcommand, ValueEnum};
use liaison_backup_local::LocalWorkspaceBackup;
use liaison_people::{CreatePerson, ListPeople, PeopleError};
use liaison_vault_markdown::MarkdownVault;
use liaison_workspace::{
    BackupError, BuildProfile, CreateWorkspaceBackup, InitialiseWorkspace,
    RestoreWorkspaceBackup, ValidateWorkspace, VerifyWorkspaceBackup, WorkspaceError,
    WorkspaceProfile, WorkspaceStore,
};
use serde::Serialize;
use serde_json::json;
use std::{path::PathBuf, process::ExitCode};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(
    name = "liaison",
    version,
    about = "Local-authoritative relationship manager",
    long_about = "Create, inspect, validate, back up, and restore an open Liaison RM workspace without requiring an account or hosted service."
)]
struct Cli {
    /// Canonical workspace directory.
    #[arg(long, global = true, default_value = ".")]
    workspace: PathBuf,

    /// Output format for commands and errors.
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Human)]
    output: OutputFormat,

    #[command(subcommand)]
    command: TopLevelCommand,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Human,
    Json,
}

#[derive(Debug, Subcommand)]
enum TopLevelCommand {
    /// Create, inspect, or validate a workspace.
    Workspace(WorkspaceArgs),
    /// Create, verify, or restore a local workspace backup.
    Backup(BackupArgs),
    /// Create or list person profiles.
    Person(PersonArgs),
}

#[derive(Debug, Args)]
struct WorkspaceArgs {
    #[command(subcommand)]
    command: WorkspaceCommand,
}

#[derive(Debug, Subcommand)]
enum WorkspaceCommand {
    /// Initialise a new local workspace.
    Init {
        #[arg(long)]
        name: String,
        #[arg(long, value_enum, default_value_t = WorkspaceProfileArg::Personal)]
        profile: WorkspaceProfileArg,
        #[arg(long, value_enum, default_value_t = BuildProfileArg::Airgap)]
        build_profile: BuildProfileArg,
        #[arg(long, default_value = "en-IE")]
        locale: String,
    },
    /// Read and display the workspace manifest.
    Inspect,
    /// Validate the manifest, required layout, and supported records.
    Validate,
}

#[derive(Debug, Args)]
struct BackupArgs {
    #[command(subcommand)]
    command: BackupCommand,
}

#[derive(Debug, Subcommand)]
enum BackupCommand {
    /// Create a new immutable local backup directory.
    Create {
        /// New backup directory. Existing paths are never overwritten.
        #[arg(long)]
        destination: PathBuf,
    },
    /// Verify every declared payload file and SHA-256 digest.
    Verify {
        /// Backup directory containing manifest.json and payload/.
        #[arg(long)]
        backup: PathBuf,
    },
    /// Restore into a new isolated directory and validate before activation.
    Restore {
        /// Verified backup directory.
        #[arg(long)]
        backup: PathBuf,
        /// New restore target. The path must not already exist.
        #[arg(long)]
        target: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum WorkspaceProfileArg {
    Personal,
    Family,
    Team,
    Workplace,
}

impl From<WorkspaceProfileArg> for WorkspaceProfile {
    fn from(value: WorkspaceProfileArg) -> Self {
        match value {
            WorkspaceProfileArg::Personal => Self::Personal,
            WorkspaceProfileArg::Family => Self::Family,
            WorkspaceProfileArg::Team => Self::Team,
            WorkspaceProfileArg::Workplace => Self::Workplace,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum BuildProfileArg {
    Airgap,
    ConnectedLocal,
}

impl From<BuildProfileArg> for BuildProfile {
    fn from(value: BuildProfileArg) -> Self {
        match value {
            BuildProfileArg::Airgap => Self::Airgap,
            BuildProfileArg::ConnectedLocal => Self::ConnectedLocal,
        }
    }
}

#[derive(Debug, Args)]
struct PersonArgs {
    #[command(subcommand)]
    command: PersonCommand,
}

#[derive(Debug, Subcommand)]
enum PersonCommand {
    /// Create a person profile as a readable Markdown record.
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: Option<String>,
    },
    /// List person profiles in stable display-name order.
    List {
        #[arg(long)]
        include_archived: bool,
    },
}

#[derive(Debug, Error)]
enum CliError {
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
    #[error(transparent)]
    Backup(#[from] BackupError),
    #[error(transparent)]
    People(#[from] PeopleError),
    #[error("failed to serialise command output: {0}")]
    Serialisation(#[from] serde_json::Error),
}

impl CliError {
    const fn code(&self) -> &'static str {
        match self {
            Self::Workspace(WorkspaceError::AlreadyExists) => "workspace.already-exists",
            Self::Workspace(WorkspaceError::NotFound) => "workspace.not-found",
            Self::Workspace(WorkspaceError::UnsupportedSchema { .. }) => {
                "workspace.unsupported-schema"
            }
            Self::Workspace(_) => "workspace.error",
            Self::Backup(BackupError::DestinationExists(_)) => "backup.destination-exists",
            Self::Backup(BackupError::DestinationInsideWorkspace(_)) => {
                "backup.destination-inside-workspace"
            }
            Self::Backup(BackupError::RestoreTargetExists(_)) => "backup.restore-target-exists",
            Self::Backup(BackupError::RestoreTargetInsideSnapshot(_)) => {
                "backup.restore-target-inside-snapshot"
            }
            Self::Backup(
                BackupError::UnexpectedFormat(_)
                | BackupError::UnsupportedFormatVersion { .. }
                | BackupError::EmptySnapshot
                | BackupError::DuplicatePath(_)
                | BackupError::UnsortedManifest
                | BackupError::UnsafePath(_)
                | BackupError::InvalidDigest(_)
                | BackupError::ManifestMissing(_)
                | BackupError::PayloadMismatch(_)
                | BackupError::ChecksumMismatch { .. }
                | BackupError::WorkspaceIdentityMismatch { .. }
                | BackupError::WorkspaceSchemaMismatch { .. }
                | BackupError::WorkspaceInvalid(_),
            ) => "backup.verification-failed",
            Self::Backup(BackupError::SymbolicLink(_)) => "backup.symbolic-link",
            Self::Backup(BackupError::CleanupFailed { .. } | BackupError::Storage(_)) => {
                "backup.storage-error"
            }
            Self::Backup(BackupError::Workspace(WorkspaceError::NotFound)) => {
                "workspace.not-found"
            }
            Self::Backup(BackupError::Workspace(_)) => "backup.workspace-error",
            Self::People(PeopleError::AlreadyExists) => "people.already-exists",
            Self::People(PeopleError::NotFound) => "people.not-found",
            Self::People(PeopleError::RevisionConflict { .. }) => "people.revision-conflict",
            Self::People(_) => "people.error",
            Self::Serialisation(_) => "cli.serialisation-error",
        }
    }

    const fn exit_code(&self) -> u8 {
        match self {
            Self::Workspace(WorkspaceError::NotFound)
            | Self::Backup(BackupError::Workspace(WorkspaceError::NotFound))
            | Self::People(PeopleError::NotFound) => 3,
            Self::Workspace(WorkspaceError::AlreadyExists)
            | Self::Backup(
                BackupError::DestinationExists(_)
                | BackupError::DestinationInsideWorkspace(_)
                | BackupError::RestoreTargetExists(_)
                | BackupError::RestoreTargetInsideSnapshot(_),
            )
            | Self::People(PeopleError::AlreadyExists | PeopleError::RevisionConflict { .. }) => 4,
            Self::Workspace(WorkspaceError::UnsupportedSchema { .. })
            | Self::Backup(
                BackupError::UnexpectedFormat(_)
                | BackupError::UnsupportedFormatVersion { .. }
                | BackupError::EmptySnapshot
                | BackupError::DuplicatePath(_)
                | BackupError::UnsortedManifest
                | BackupError::UnsafePath(_)
                | BackupError::InvalidDigest(_)
                | BackupError::SymbolicLink(_)
                | BackupError::ManifestMissing(_)
                | BackupError::PayloadMismatch(_)
                | BackupError::ChecksumMismatch { .. }
                | BackupError::WorkspaceIdentityMismatch { .. }
                | BackupError::WorkspaceSchemaMismatch { .. }
                | BackupError::WorkspaceInvalid(_),
            ) => 5,
            Self::Workspace(_)
            | Self::Backup(_)
            | Self::People(_)
            | Self::Serialisation(_) => 1,
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let output = cli.output;
    match execute(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            write_error(output, &error);
            ExitCode::from(error.exit_code())
        }
    }
}

fn execute(cli: Cli) -> Result<(), CliError> {
    let vault = MarkdownVault::new();
    let backup_store = LocalWorkspaceBackup::new();
    match cli.command {
        TopLevelCommand::Workspace(args) => match args.command {
            WorkspaceCommand::Init {
                name,
                profile,
                build_profile,
                locale,
            } => {
                let manifest = InitialiseWorkspace::new(vault).execute(
                    &cli.workspace,
                    name,
                    profile.into(),
                    build_profile.into(),
                    locale,
                )?;
                let human = format!(
                    "Initialised workspace '{}' at {}",
                    manifest.name,
                    cli.workspace.display()
                );
                write_success(cli.output, &human, &manifest)
            }
            WorkspaceCommand::Inspect => {
                let manifest = vault.load(&cli.workspace)?;
                let human = format!(
                    "Workspace '{}' ({}) uses schema {}",
                    manifest.name, manifest.workspace_id, manifest.schema_version
                );
                write_success(cli.output, &human, &manifest)
            }
            WorkspaceCommand::Validate => {
                let report = ValidateWorkspace::new(vault).execute(&cli.workspace)?;
                let summary = if report.is_valid() {
                    format!(
                        "Workspace {} is valid with {} finding(s)",
                        report.workspace_id,
                        report.findings.len()
                    )
                } else {
                    format!(
                        "Workspace {} is invalid with {} finding(s)",
                        report.workspace_id,
                        report.findings.len()
                    )
                };
                write_success(cli.output, &summary, &report)
            }
        },
        TopLevelCommand::Backup(args) => match args.command {
            BackupCommand::Create { destination } => {
                let manifest =
                    CreateWorkspaceBackup::new(vault, backup_store).execute(
                        &cli.workspace,
                        &destination,
                    )?;
                let human = format!(
                    "Created backup for workspace {} at {} with {} file(s)",
                    manifest.workspace_id,
                    destination.display(),
                    manifest.files.len()
                );
                write_success(cli.output, &human, &manifest)
            }
            BackupCommand::Verify { backup } => {
                let report = VerifyWorkspaceBackup::new(backup_store).execute(&backup)?;
                let human = format!(
                    "Verified backup for workspace {}: {} file(s), {} byte(s)",
                    report.workspace_id, report.files_checked, report.total_bytes
                );
                write_success(cli.output, &human, &report)
            }
            BackupCommand::Restore { backup, target } => {
                let report = RestoreWorkspaceBackup::new(vault, backup_store)
                    .execute(&backup, &target)?;
                let human = format!(
                    "Restored workspace {} to {} with {} file(s)",
                    report.workspace_id, report.target, report.files_restored
                );
                write_success(cli.output, &human, &report)
            }
        },
        TopLevelCommand::Person(args) => match args.command {
            PersonCommand::Create { name, email } => {
                let person = CreatePerson::new(vault).execute(&cli.workspace, name, email)?;
                let human = format!("Created person '{}' ({})", person.display_name, person.id);
                write_success(cli.output, &human, &person)
            }
            PersonCommand::List { include_archived } => {
                let people = ListPeople::new(vault).execute(&cli.workspace, include_archived)?;
                let human = if people.is_empty() {
                    "No people found".to_owned()
                } else {
                    people
                        .iter()
                        .map(|person| {
                            let state = if person.archived {
                                "archived"
                            } else {
                                "active"
                            };
                            format!("{}\t{}\t{}", person.id, person.display_name, state)
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                write_success(cli.output, &human, &people)
            }
        },
    }
}

fn write_success<T>(output: OutputFormat, human: &str, value: &T) -> Result<(), CliError>
where
    T: Serialize,
{
    match output {
        OutputFormat::Human => println!("{human}"),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(value)?),
    }
    Ok(())
}

fn write_error(output: OutputFormat, error: &CliError) {
    match output {
        OutputFormat::Human => eprintln!("{}: {error}", error.code()),
        OutputFormat::Json => {
            let value = json!({
                "error": {
                    "code": error.code(),
                    "message": error.to_string(),
                    "exit_code": error.exit_code()
                }
            });
            match serde_json::to_string_pretty(&value) {
                Ok(rendered) => eprintln!("{rendered}"),
                Err(serialisation_error) => eprintln!(
                    "cli.serialisation-error: could not render original error '{error}': {serialisation_error}"
                ),
            }
        }
    }
}
