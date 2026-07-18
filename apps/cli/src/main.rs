use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand, ValueEnum};
use liaison_people::{CreatePerson, ListPeople, PeopleError};
use liaison_relationship_yaml::RelationshipYamlStore;
use liaison_relationships::{
    ContactCadence, GetRelationship, ListRelationships, MaintenanceStatus,
    RelationshipError, RelationshipProfile, RelationshipTier, RelationshipUpdate,
    SaveRelationship,
};
use liaison_shared_kernel::{PersonId, Revision};
use liaison_vault_markdown::MarkdownVault;
use liaison_workspace::{
    BuildProfile, InitialiseWorkspace, ValidateWorkspace, WorkspaceError,
    WorkspaceProfile, WorkspaceStore,
};
use serde::Serialize;
use serde_json::json;
use std::{collections::BTreeSet, path::PathBuf, process::ExitCode};
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(
    name = "liaison",
    version,
    about = "Local-authoritative relationship manager",
    long_about = "Create, inspect, and validate an open Liaison RM workspace and record explicit relationship intent without requiring an account or hosted service."
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
    /// Inspect or configure explicit relationship intent.
    Relationship(RelationshipArgs),
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

#[derive(Debug, Args)]
struct RelationshipArgs {
    #[command(subcommand)]
    command: RelationshipCommand,
}

#[derive(Debug, Subcommand)]
enum RelationshipCommand {
    /// Show relationship intent for one Person.
    Show {
        #[arg(long)]
        person_id: PersonId,
        #[arg(long)]
        as_of: Option<NaiveDate>,
    },
    /// List relationship intent records in stable Person-ID order.
    List {
        #[arg(long)]
        as_of: Option<NaiveDate>,
    },
    /// Create or update relationship intent. Omitted fields retain existing values.
    Set(RelationshipSetArgs),
}

#[derive(Debug, Args)]
struct RelationshipSetArgs {
    #[arg(long)]
    person_id: PersonId,
    /// Required for updates; omit only when creating the first record.
    #[arg(long)]
    expected_revision: Option<u64>,
    #[arg(long)]
    relationship_type: Option<String>,
    #[arg(long)]
    clear_relationship_type: bool,
    #[arg(long, value_enum)]
    tier: Option<RelationshipTierArg>,
    #[arg(long, value_enum)]
    cadence: Option<ContactCadenceArg>,
    #[arg(long, requires = "cadence")]
    custom_days: Option<u16>,
    #[arg(long)]
    last_contacted: Option<NaiveDate>,
    #[arg(long)]
    clear_last_contacted: bool,
    #[arg(long)]
    next_contact_due: Option<NaiveDate>,
    #[arg(long)]
    clear_next_contact_due: bool,
    #[arg(long)]
    reason: Option<String>,
    #[arg(long)]
    clear_reason: bool,
    #[arg(long)]
    topic: Option<String>,
    #[arg(long)]
    clear_topic: bool,
    /// Replace circles. Repeat the flag for more than one circle.
    #[arg(long = "circle")]
    circles: Vec<String>,
    #[arg(long)]
    clear_circles: bool,
    #[arg(long)]
    paused_until: Option<NaiveDate>,
    #[arg(long)]
    clear_pause: bool,
    #[arg(long, conflicts_with = "allow_contact")]
    do_not_contact: bool,
    #[arg(long, conflicts_with = "do_not_contact")]
    allow_contact: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RelationshipTierArg {
    Core,
    Active,
    Warm,
    Loose,
    Paused,
    Archive,
}

impl From<RelationshipTierArg> for RelationshipTier {
    fn from(value: RelationshipTierArg) -> Self {
        match value {
            RelationshipTierArg::Core => Self::Core,
            RelationshipTierArg::Active => Self::Active,
            RelationshipTierArg::Warm => Self::Warm,
            RelationshipTierArg::Loose => Self::Loose,
            RelationshipTierArg::Paused => Self::Paused,
            RelationshipTierArg::Archive => Self::Archive,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ContactCadenceArg {
    None,
    Monthly,
    Quarterly,
    TwiceYearly,
    Yearly,
    Custom,
}

#[derive(Debug, Serialize)]
struct RelationshipView {
    relationship: RelationshipProfile,
    maintenance_status: Option<MaintenanceStatus>,
}

#[derive(Debug, Error)]
enum CliError {
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
    #[error(transparent)]
    People(#[from] PeopleError),
    #[error(transparent)]
    Relationship(#[from] RelationshipError),
    #[error("revision must be a positive integer")]
    InvalidRevision,
    #[error("{0}")]
    InvalidArguments(String),
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
            Self::Relationship(RelationshipError::AlreadyExists) => {
                "relationship.already-exists"
            }
            Self::Relationship(RelationshipError::NotFound) => "relationship.not-found",
            Self::Relationship(RelationshipError::RevisionConflict { .. }) => {
                "relationship.revision-conflict"
            }
            Self::Relationship(RelationshipError::InvalidCustomCadence(_)) => {
                "relationship.invalid-cadence"
            }
            Self::Relationship(_) => "relationship.error",
            Self::InvalidRevision => "cli.invalid-revision",
            Self::InvalidArguments(_) => "cli.invalid-arguments",
            Self::Serialisation(_) => "cli.serialisation-error",
        }
    }

    const fn exit_code(&self) -> u8 {
        match self {
            Self::Workspace(WorkspaceError::NotFound)
            | Self::People(PeopleError::NotFound)
            | Self::Relationship(RelationshipError::NotFound) => 3,
            Self::Workspace(WorkspaceError::AlreadyExists)
            | Self::People(PeopleError::AlreadyExists | PeopleError::RevisionConflict { .. })
            | Self::Relationship(
                RelationshipError::AlreadyExists | RelationshipError::RevisionConflict { .. },
            ) => 4,
            Self::Workspace(WorkspaceError::UnsupportedSchema { .. })
            | Self::Relationship(RelationshipError::InvalidCustomCadence(_))
            | Self::InvalidRevision
            | Self::InvalidArguments(_) => 5,
            Self::Workspace(_)
            | Self::People(_)
            | Self::Relationship(_)
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

#[allow(clippy::too_many_lines)]
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
                            let state = if person.archived { "archived" } else { "active" };
                            format!("{}\t{}\t{}", person.id, person.display_name, state)
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                write_success(cli.output, &human, &people)
            }
        },
        TopLevelCommand::Relationship(args) => {
            execute_relationship(cli.output, &cli.workspace, args)
        }
    }
}

fn execute_relationship(
    output: OutputFormat,
    workspace: &std::path::Path,
    args: RelationshipArgs,
) -> Result<(), CliError> {
    let store = RelationshipYamlStore::new();
    match args.command {
        RelationshipCommand::Show { person_id, as_of } => {
            let relationship = GetRelationship::new(store).execute(workspace, person_id)?;
            let view = relationship.map(|relationship| RelationshipView {
                maintenance_status: as_of.map(|date| {
                    relationship.maintenance_status(date, false)
                }),
                relationship,
            });
            let human = view.as_ref().map_or_else(
                || format!("No relationship intent recorded for {person_id}"),
                |view| relationship_line(view),
            );
            write_success(output, &human, &view)
        }
        RelationshipCommand::List { as_of } => {
            let relationships = ListRelationships::new(store).execute(workspace)?;
            let views = relationships
                .into_iter()
                .map(|relationship| RelationshipView {
                    maintenance_status: as_of.map(|date| {
                        relationship.maintenance_status(date, false)
                    }),
                    relationship,
                })
                .collect::<Vec<_>>();
            let human = if views.is_empty() {
                "No relationship intent records found".to_owned()
            } else {
                views
                    .iter()
                    .map(relationship_line)
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            write_success(output, &human, &views)
        }
        RelationshipCommand::Set(set) => {
            let existing = GetRelationship::new(store.clone())
                .execute(workspace, set.person_id)?;
            let update = merge_relationship_update(existing.as_ref(), &set)?;
            let expected = set
                .expected_revision
                .map(|value| Revision::new(value).ok_or(CliError::InvalidRevision))
                .transpose()?;
            let relationship = SaveRelationship::new(store).execute(
                workspace,
                set.person_id,
                expected,
                update,
            )?;
            let human = format!(
                "Saved relationship intent for {} at revision {}",
                relationship.person_id,
                relationship.revision.get()
            );
            write_success(output, &human, &relationship)
        }
    }
}

fn merge_relationship_update(
    existing: Option<&RelationshipProfile>,
    set: &RelationshipSetArgs,
) -> Result<RelationshipUpdate, CliError> {
    let mut update = existing.map_or_else(RelationshipUpdate::default, |profile| {
        RelationshipUpdate {
            relationship_type: profile.relationship_type.clone(),
            tier: profile.tier,
            cadence: profile.cadence,
            last_contacted: profile.last_contacted,
            next_contact_due: profile.next_contact_due,
            reason_to_contact: profile.reason_to_contact.clone(),
            last_meaningful_topic: profile.last_meaningful_topic.clone(),
            circles: profile.circles.clone(),
            paused_until: profile.paused_until,
            do_not_contact: profile.do_not_contact,
        }
    });

    if set.clear_relationship_type {
        update.relationship_type = None;
    } else if let Some(value) = &set.relationship_type {
        update.relationship_type = Some(value.clone());
    }
    if let Some(value) = set.tier {
        update.tier = value.into();
    }
    if let Some(value) = set.cadence {
        update.cadence = cadence(value, set.custom_days)?;
    } else if set.custom_days.is_some() {
        return Err(CliError::InvalidArguments(
            "--custom-days requires --cadence custom".to_owned(),
        ));
    }
    if set.clear_last_contacted {
        update.last_contacted = None;
    } else if let Some(value) = set.last_contacted {
        update.last_contacted = Some(value);
    }
    if set.clear_next_contact_due {
        update.next_contact_due = None;
    } else if let Some(value) = set.next_contact_due {
        update.next_contact_due = Some(value);
    }
    if set.clear_reason {
        update.reason_to_contact = None;
    } else if let Some(value) = &set.reason {
        update.reason_to_contact = Some(value.clone());
    }
    if set.clear_topic {
        update.last_meaningful_topic = None;
    } else if let Some(value) = &set.topic {
        update.last_meaningful_topic = Some(value.clone());
    }
    if set.clear_circles {
        update.circles.clear();
    } else if !set.circles.is_empty() {
        update.circles = set.circles.iter().cloned().collect::<BTreeSet<_>>();
    }
    if set.clear_pause {
        update.paused_until = None;
    } else if let Some(value) = set.paused_until {
        update.paused_until = Some(value);
    }
    if set.do_not_contact {
        update.do_not_contact = true;
    } else if set.allow_contact {
        update.do_not_contact = false;
    }
    Ok(update)
}

fn cadence(
    cadence: ContactCadenceArg,
    custom_days: Option<u16>,
) -> Result<ContactCadence, CliError> {
    match cadence {
        ContactCadenceArg::None => Ok(ContactCadence::None),
        ContactCadenceArg::Monthly => Ok(ContactCadence::Monthly),
        ContactCadenceArg::Quarterly => Ok(ContactCadence::Quarterly),
        ContactCadenceArg::TwiceYearly => Ok(ContactCadence::TwiceYearly),
        ContactCadenceArg::Yearly => Ok(ContactCadence::Yearly),
        ContactCadenceArg::Custom => custom_days
            .map(ContactCadence::Custom)
            .ok_or_else(|| {
                CliError::InvalidArguments(
                    "--cadence custom requires --custom-days".to_owned(),
                )
            }),
    }
}

fn relationship_line(view: &RelationshipView) -> String {
    let status = view
        .maintenance_status
        .map_or_else(|| "not calculated".to_owned(), |value| format!("{value:?}"));
    format!(
        "{}\t{:?}\t{:?}\t{}",
        view.relationship.person_id,
        view.relationship.tier,
        view.relationship.cadence,
        status
    )
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
