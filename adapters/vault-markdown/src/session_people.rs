use super::{
    BoundMarkdownVault, bound_create_person, bound_find_person, bound_list_people,
    bound_save_person,
};
use liaison_people::{PeopleError, PersonProfile, PersonRepository};
use liaison_shared_kernel::{PersonId, Revision};
use liaison_workspace::{OperationContext, WorkspaceWorkGuard};

/// A People repository that exists only while one session work guard is live.
///
/// The concrete adapter is private so callers cannot pair a writer guard for
/// one workspace with repositories bound to another workspace. Mutation
/// authority is also explicit: read-only repositories cannot create or save.
#[derive(Debug)]
struct SessionPeopleRepository<'work, 'session> {
    work: &'work WorkspaceWorkGuard<'session, BoundMarkdownVault>,
    mutation_context: Option<OperationContext>,
}

#[must_use]
pub fn people_repository<'work, 'session>(
    work: &'work WorkspaceWorkGuard<'session, BoundMarkdownVault>,
) -> impl PersonRepository + 'work
where
    'session: 'work,
{
    SessionPeopleRepository {
        work,
        mutation_context: None,
    }
}

#[must_use]
pub fn people_mutation_repository<'work, 'session>(
    work: &'work WorkspaceWorkGuard<'session, BoundMarkdownVault>,
    mutation_context: OperationContext,
) -> impl PersonRepository + 'work
where
    'session: 'work,
{
    SessionPeopleRepository {
        work,
        mutation_context: Some(mutation_context),
    }
}

impl PersonRepository for SessionPeopleRepository<'_, '_> {
    fn create(&self, person: &PersonProfile) -> Result<(), PeopleError> {
        let context = self.mutation_context.ok_or_else(|| {
            PeopleError::Storage(
                "person mutation requires an application-provided recoverable operation context"
                    .to_owned(),
            )
        })?;
        bound_create_person(
            &self.work.repositories().root,
            self.work.workspace_id(),
            context,
            person,
        )
    }

    fn list(&self, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError> {
        bound_list_people(&self.work.repositories().root, include_archived)
    }

    fn find(&self, id: PersonId) -> Result<PersonProfile, PeopleError> {
        let (_, parsed) = bound_find_person(&self.work.repositories().root, id)?;
        parsed.document.into_domain()
    }

    fn save(&self, person: &PersonProfile, expected_revision: Revision) -> Result<(), PeopleError> {
        let context = self.mutation_context.ok_or_else(|| {
            PeopleError::Storage(
                "person mutation requires an application-provided recoverable operation context"
                    .to_owned(),
            )
        })?;
        bound_save_person(
            &self.work.repositories().root,
            self.work.workspace_id(),
            context,
            person,
            expected_revision,
        )
    }
}
