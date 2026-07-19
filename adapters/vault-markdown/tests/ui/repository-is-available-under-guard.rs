use liaison_people::PersonRepository;
use liaison_vault_markdown::{BoundMarkdownVault, people_repository};
use liaison_workspace::WorkspaceWorkGuard;

fn require_repository<Repository: PersonRepository>(_repository: &Repository) {}

fn available<'work, 'session>(
    work: &'work WorkspaceWorkGuard<'session, BoundMarkdownVault>,
) where
    'session: 'work,
{
    let repository = people_repository(work);
    require_repository(&repository);
}

fn main() {}
