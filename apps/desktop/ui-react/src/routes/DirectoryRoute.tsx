import { useEffect, useMemo, useRef, useState, type FormEvent } from "react";
import type { PersonSummary } from "../application-contract";
import { Button, Field, Surface } from "../components";
import type { DesktopController } from "../useDesktopController";

export interface DirectoryRouteProps {
  readonly controller: DesktopController;
}

export function DirectoryRoute({ controller }: DirectoryRouteProps) {
  const [displayName, setDisplayName] = useState("");
  const [email, setEmail] = useState("");
  const [query, setQuery] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const profileHeading = useRef<HTMLHeadingElement>(null);

  const visiblePeople = useMemo(() => {
    const normalized = query.trim().toLocaleLowerCase("en-IE");
    if (normalized === "") {
      return controller.people;
    }
    return controller.people.filter((person) =>
      [person.displayName, person.primaryEmail ?? "", person.primaryPhone ?? ""]
        .join(" ")
        .toLocaleLowerCase("en-IE")
        .includes(normalized),
    );
  }, [controller.people, query]);

  const selected = useMemo(
    () => controller.people.find((person) => person.personId === selectedId) ?? null,
    [controller.people, selectedId],
  );

  useEffect(() => {
    if (selected !== null) {
      profileHeading.current?.focus();
    }
  }, [selected]);

  async function submit(event: FormEvent<HTMLFormElement>): Promise<void> {
    event.preventDefault();
    const name = displayName.trim();
    if (name === "") {
      return;
    }
    const person = await controller.createPerson({
      displayName: name,
      ...(email.trim() === "" ? {} : { email: email.trim() }),
    });
    if (person !== null) {
      setDisplayName("");
      setEmail("");
      setSelectedId(person.personId);
    }
  }

  if (controller.workspace === null) {
    return (
      <Surface aria-labelledby="route-heading" emphasis="raised">
        <p className="section-label">People</p>
        <h2 id="route-heading" tabIndex={-1}>
          Directory
        </h2>
        <p>Open a local workspace in Settings before reading or creating Person records.</p>
      </Surface>
    );
  }

  return (
    <Surface aria-labelledby="route-heading" emphasis="raised">
      <p className="section-label">People</p>
      <h2 id="route-heading" tabIndex={-1}>
        Directory
      </h2>
      <p>
        Person records are readable local files. Search narrows the currently loaded list and does not change canonical records.
      </p>

      <form className="person-form" onSubmit={(event) => void submit(event)}>
        <Field
          autoComplete="name"
          disabled={controller.busy}
          label="Display name"
          name="displayName"
          onChange={(event) => setDisplayName(event.currentTarget.value)}
          required
          value={displayName}
        />
        <Field
          autoComplete="email"
          disabled={controller.busy}
          label="Email address (optional)"
          name="email"
          onChange={(event) => setEmail(event.currentTarget.value)}
          type="email"
          value={email}
        />
        <div className="action-row">
          <Button busy={controller.busy} busyLabel="Saving…" tone="primary" type="submit">
            Add person
          </Button>
          <Button
            busy={controller.busy}
            busyLabel="Refreshing…"
            onClick={() => void controller.refreshPeople()}
          >
            Refresh Directory
          </Button>
        </div>
      </form>

      <div className="directory-layout">
        <section aria-labelledby="people-list-heading">
          <h3 id="people-list-heading">Stored people</h3>
          <Field
            label="Search people"
            name="peopleSearch"
            onChange={(event) => setQuery(event.currentTarget.value)}
            type="search"
            value={query}
          />
          <p aria-live="polite" className="result-count">
            {visiblePeople.length} {visiblePeople.length === 1 ? "person" : "people"} shown
          </p>
          {visiblePeople.length === 0 ? (
            <p>No readable Person record matches this search.</p>
          ) : (
            <ul className="person-list">
              {visiblePeople.map((person) => (
                <li key={person.personId}>
                  <button
                    aria-pressed={person.personId === selectedId}
                    className="person-row"
                    onClick={() => setSelectedId(person.personId)}
                    type="button"
                  >
                    <span>{person.displayName}</span>
                    <small>{person.primaryEmail ?? "No email recorded"}</small>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </section>

        <ProfilePanel
          onClose={() => setSelectedId(null)}
          person={selected}
          profileHeading={profileHeading}
        />
      </div>
    </Surface>
  );
}

interface ProfilePanelProps {
  readonly person: PersonSummary | null;
  readonly onClose: () => void;
  readonly profileHeading: React.RefObject<HTMLHeadingElement>;
}

function ProfilePanel({ person, onClose, profileHeading }: ProfilePanelProps) {
  if (person === null) {
    return (
      <aside aria-label="Person profile" className="profile-panel profile-panel--empty">
        <p>Select a person to read the stored summary.</p>
      </aside>
    );
  }

  return (
    <aside aria-labelledby="profile-heading" className="profile-panel">
      <div className="profile-panel__heading">
        <h3 id="profile-heading" ref={profileHeading} tabIndex={-1}>
          {person.displayName}
        </h3>
        <Button onClick={onClose} tone="quiet">
          Close profile
        </Button>
      </div>
      <dl className="record-summary">
        <div>
          <dt>Email</dt>
          <dd>{person.primaryEmail ?? "Not recorded"}</dd>
        </div>
        <div>
          <dt>Phone</dt>
          <dd>{person.primaryPhone ?? "Not recorded"}</dd>
        </div>
        <div>
          <dt>Birthday</dt>
          <dd>{person.birthday ?? "Not recorded"}</dd>
        </div>
        <div>
          <dt>Revision</dt>
          <dd>{person.revision}</dd>
        </div>
        <div>
          <dt>Identifier</dt>
          <dd><code>{person.personId}</code></dd>
        </div>
      </dl>
    </aside>
  );
}
