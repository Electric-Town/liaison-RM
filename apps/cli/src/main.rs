use clap::{Args, Parser, Subcommand, ValueEnum};
use liaison_people::{CreatePerson, ListPeople, PeopleError};
use liaison_vault_markdown::MarkdownVault;
use liaison_workspace::{
    BuildProfile, InitialiseWorkspace, ValidateWorkspace, WorkspaceError, WorkspaceProfile,
    WorkspaceStore,
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
    long_about = "Create, inspect, and validate an open Liaison RM workspace without requiring an account or hosted service."
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
            Self::People(PeopleError::AlreadyExists) => "people.already-exists",
            Self::People(PeopleError::NotFound) => "people.not-found",
            Self::People(PeopleError::RevisionConflict { .. }) => "people.revision-conflict",
            Self::People(_) => "people.error",
            Self::Serialisation(_) => "cli.serialisation-error",
        }
    }

    const fn exit_code(&self) -> u8 {
        match self {
            Self::Workspace(WorkspaceError::NotFound) | Self::People(PeopleError::NotFound) => 3,
            Self::Workspace(WorkspaceError::AlreadyExists)
            | Self::People(PeopleError::AlreadyExists | PeopleError::RevisionConflict { .. }) => 4,
            Self::Workspace(WorkspaceError::UnsupportedSchema { .. }) => 5,
            Self::Workspace(_) | Self::People(_) | Self::Serialisation(_) => 1,
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
