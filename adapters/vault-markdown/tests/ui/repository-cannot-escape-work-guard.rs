use liaison_people::PersonRepository;
use liaison_vault_markdown::{BoundMarkdownVault, people_repository};
use liaison_workspace::WorkspaceWorkGuard;

fn escape<'work, 'session>(
    work: &'work WorkspaceWorkGuard<'session, BoundMarkdownVault>,
) -> impl PersonRepository + 'static
where
    'session: 'work,
{
    people_repository(work)
}

fn main() {}
