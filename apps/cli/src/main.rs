use clap::{Args, Parser, Subcommand, ValueEnum};
use liaison_application::{
    APPLICATION_CONTRACT_VERSION, ApplicationError, BuildProfile, CommandResult,
    CreatePersonCommand, FindingSeverityDto, InitialiseWorkspaceCommand, LiaisonApplication,
    ListPeopleQuery, OpenWorkspaceCommand, WorkspaceDto, WorkspaceOpenDto, WorkspaceProfile,
    WorkspaceSessionCommand, WorkspaceValidationDto,
};
use serde::Serialize;
use serde_json::json;
use std::{collections::BTreeMap, path::PathBuf, process::ExitCode};
use thiserror::Error;

const EXIT_SUCCESS: u8 = 0;
const EXIT_GENERAL_ERROR: u8 = 1;
const EXIT_NOT_FOUND: u8 = 3;
const EXIT_CONFLICT: u8 = 4;
const EXIT_UNSUPPORTED: u8 = 5;
const EXIT_VALIDATION_INVALID: u8 = 6;

#[derive(Debug, Parser)]
#[command(
    name = "liaison",
    version,
    about = "Local-authoritative relationship manager",
    long_about = "Create, inspect, and validate an open Liaison RM workspace without requiring an account or hosted service."
)]
struct Cli {
    /// Explicit canonical workspace directory.
    #[arg(long, value_name = "PATH")]
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
        #[arg(long, value_enum, default_value_t = WorkspaceProfileArg::Workplace)]
        profile: WorkspaceProfileArg,
        /// Workspace policy declaration; it is not artifact-level Airgap evidence.
        #[arg(long, value_enum, default_value_t = BuildProfileArg::ConnectedLocal)]
        build_profile: BuildProfileArg,
        #[arg(long, default_value = "en-IE")]
        locale: String,
    },
    /// Open a typed application session and display its workspace identity.
    Inspect,
    /// Open a typed application session and validate the workspace.
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
    Application(#[from] ApplicationError),
    #[error("failed to serialise command output: {0}")]
    Serialisation(#[from] serde_json::Error),
}

impl CliError {
    fn exit_code(&self) -> u8 {
        match self {
            Self::Application(error) => application_exit_code(&error.code),
            Self::Serialisation(_) => EXIT_GENERAL_ERROR,
        }
    }
}

fn application_exit_code(code: &str) -> u8 {
    match code {
        "workspace.not-found" | "people.not-found" | "application.workspace-session-not-found" => {
            EXIT_NOT_FOUND
        }
        "workspace.already-exists"
        | "workspace.initialise-target-not-empty"
        | "people.already-exists"
        | "people.revision-conflict"
        | "application.workspace-session-stale" => EXIT_CONFLICT,
        "workspace.unsupported-schema" => EXIT_UNSUPPORTED,
        _ => EXIT_GENERAL_ERROR,
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let output = cli.output;
    match execute(cli) {
        Ok(exit_code) => ExitCode::from(exit_code),
        Err(error) => {
            write_error(output, &error);
            ExitCode::from(error.exit_code())
        }
    }
}

fn execute(cli: Cli) -> Result<u8, CliError> {
    let application = LiaisonApplication::new();
    let workspace_path = cli.workspace.to_string_lossy().into_owned();
    match cli.command {
        TopLevelCommand::Workspace(args) => {
            execute_workspace(&application, workspace_path, cli.output, args.command)
        }
        TopLevelCommand::Person(args) => {
            execute_person(&application, workspace_path, cli.output, args.command)
        }
    }
}

fn execute_workspace(
    application: &LiaisonApplication,
    workspace_path: String,
    output: OutputFormat,
    command: WorkspaceCommand,
) -> Result<u8, CliError> {
    match command {
        WorkspaceCommand::Init {
            name,
            profile,
            build_profile,
            locale,
        } => {
            let opened = application.initialise_workspace(InitialiseWorkspaceCommand {
                path: workspace_path,
                name,
                profile: profile.into(),
                build_profile: build_profile.into(),
                locale,
            })?;
            let workspace = workspace_result(opened);
            let human = format!(
                "Initialised workspace '{}' at {}",
                workspace.value.name, workspace.value.path
            );
            write_success(output, &human, &workspace)?;
            Ok(EXIT_SUCCESS)
        }
        WorkspaceCommand::Inspect => {
            let opened = open_workspace(application, workspace_path)?;
            let workspace = workspace_result(opened);
            let human = format!(
                "Workspace '{}' ({}) uses schema {}",
                workspace.value.name, workspace.value.workspace_id, workspace.value.schema_version
            );
            write_success(output, &human, &workspace)?;
            Ok(EXIT_SUCCESS)
        }
        WorkspaceCommand::Validate => {
            let opened = open_workspace(application, workspace_path)?;
            let report = application.validate_workspace(WorkspaceSessionCommand {
                session_id: opened.value.workspace.session_id,
            })?;
            let validity = if report.value.valid {
                "valid"
            } else {
                "invalid"
            };
            let human = validation_human(&report, validity);
            let exit_code = if report.value.valid {
                EXIT_SUCCESS
            } else {
                EXIT_VALIDATION_INVALID
            };
            write_success(output, &human, &report)?;
            Ok(exit_code)
        }
    }
}

fn validation_human(report: &CommandResult<WorkspaceValidationDto>, validity: &str) -> String {
    let mut lines = vec![
        format!(
            "Workspace {} is {validity} with {} finding(s)",
            report.value.workspace_id,
            report.value.findings.len()
        ),
        format!("Contract version: {}", report.contract_version),
        format!("Correlation ID: {}", report.command_id),
    ];
    for finding in &report.value.findings {
        let severity = match finding.severity {
            FindingSeverityDto::Info => "INFO",
            FindingSeverityDto::Warning => "WARNING",
            FindingSeverityDto::Error => "ERROR",
        };
        lines.extend([
            String::new(),
            format!("- {severity} {} [{}]", finding.code, finding.path),
            format!("  {}", finding.message),
            format!("  Recovery: {}", finding.recovery),
        ]);
    }
    lines.join("\n")
}

fn execute_person(
    application: &LiaisonApplication,
    workspace_path: String,
    output: OutputFormat,
    command: PersonCommand,
) -> Result<u8, CliError> {
    let opened = open_workspace(application, workspace_path)?;
    let session_id = opened.value.workspace.session_id;
    match command {
        PersonCommand::Create { name, email } => {
            let person = application.create_person(CreatePersonCommand {
                session_id,
                display_name: name,
                email,
            })?;
            let human = format!(
                "Created person '{}' ({})",
                person.value.display_name, person.value.id
            );
            write_success(output, &human, &person)?;
            Ok(EXIT_SUCCESS)
        }
        PersonCommand::List { include_archived } => {
            let people = application.list_people(ListPeopleQuery {
                session_id,
                include_archived,
            })?;
            let human = if people.value.is_empty() {
                "No people found".to_owned()
            } else {
                people
                    .value
                    .iter()
                    .map(|person| {
                        let state = if person.archived {
                            "archived"
                        } else {
                            "active"
                        };
                        format!("{}\t{}\t{state}", person.id, person.display_name)
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            write_success(output, &human, &people)?;
            Ok(EXIT_SUCCESS)
        }
    }
}

fn open_workspace(
    application: &LiaisonApplication,
    path: String,
) -> Result<CommandResult<WorkspaceOpenDto>, ApplicationError> {
    application.open_workspace(OpenWorkspaceCommand { path })
}

fn workspace_result(opened: CommandResult<WorkspaceOpenDto>) -> CommandResult<WorkspaceDto> {
    CommandResult {
        contract_version: opened.contract_version,
        command_id: opened.command_id,
        completed_at: opened.completed_at,
        value: opened.value.workspace,
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
    match (output, error) {
        (OutputFormat::Human, CliError::Application(error)) => {
            let details = serde_json::to_string(&error.details)
                .unwrap_or_else(|_| "{\"rendering\":\"unavailable\"}".to_owned());
            eprintln!(
                "{}: {}\nRecovery: {}\nDetails: {}\nCorrelation ID: {}",
                error.code, error.message, error.recovery, details, error.correlation_id
            );
        }
        (OutputFormat::Json, CliError::Application(error)) => {
            let value = json!({
                "error": error,
                "exit_code": application_exit_code(&error.code)
            });
            if let Ok(rendered) = serde_json::to_string_pretty(&value) {
                eprintln!("{rendered}");
            } else {
                eprintln!("cli.serialisation-error: application error could not be rendered");
            }
        }
        (OutputFormat::Human, CliError::Serialisation(_)) => {
            eprintln!("cli.serialisation-error: command output could not be rendered");
        }
        (OutputFormat::Json, CliError::Serialisation(_)) => {
            let value = json!({
                "error": {
                    "contract_version": APPLICATION_CONTRACT_VERSION,
                    "code": "cli.serialisation-error",
                    "message": "command output could not be rendered",
                    "recovery": "retry with human output and preserve the workspace if the problem repeats",
                    "details": BTreeMap::<String, serde_json::Value>::new(),
                    "correlation_id": serde_json::Value::Null
                },
                "exit_code": EXIT_GENERAL_ERROR
            });
            if let Ok(rendered) = serde_json::to_string_pretty(&value) {
                eprintln!("{rendered}");
            }
        }
    }
}
