use super::{
    BoundMarkdownVault, bound_create_person, bound_find_person, bound_list_people,
    bound_save_person,
};
use liaison_people::{PeopleError, PersonProfile, PersonRepository};
use liaison_shared_kernel::{PersonId, Revision};
use liaison_workspace::WorkspaceWorkGuard;

/// A People repository that exists only while one session work guard is live.
///
/// The concrete adapter is private so callers cannot pair a writer guard for
/// one workspace with repositories bound to another workspace. The opaque
/// return also prevents this capability from outliving the guard that proves
/// the session still owns writer authority.
#[derive(Debug)]
struct SessionPeopleRepository<'work, 'session> {
    work: &'work WorkspaceWorkGuard<'session, BoundMarkdownVault>,
}

#[must_use]
pub fn people_repository<'work, 'session>(
    work: &'work WorkspaceWorkGuard<'session, BoundMarkdownVault>,
) -> impl PersonRepository + 'work
where
    'session: 'work,
{
    SessionPeopleRepository { work }
}

impl PersonRepository for SessionPeopleRepository<'_, '_> {
    fn create(&self, person: &PersonProfile) -> Result<(), PeopleError> {
        bound_create_person(&self.work.repositories().root, person)
    }

    fn list(&self, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError> {
        bound_list_people(&self.work.repositories().root, include_archived)
    }

    fn find(&self, id: PersonId) -> Result<PersonProfile, PeopleError> {
        let (_, parsed) = bound_find_person(&self.work.repositories().root, id)?;
        parsed.document.into_domain()
    }

    fn save(&self, person: &PersonProfile, expected_revision: Revision) -> Result<(), PeopleError> {
        bound_save_person(&self.work.repositories().root, person, expected_revision)
    }
}
