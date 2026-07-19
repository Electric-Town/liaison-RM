use liaison_people::PersonRepository;
use liaison_vault_markdown::BoundMarkdownVault;

fn require_repository<Repository: PersonRepository>() {}

fn main() {
    require_repository::<BoundMarkdownVault>();
}
