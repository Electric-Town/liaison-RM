import { EVENT_STAGES } from "../application-contract";
import { Surface } from "../components";

const stageLabels = {
  details: "Details",
  cohort: "Cohort",
  attendees: "Attendees",
  readiness: "Readiness",
  brief: "Brief",
} as const;

export function EventsRoute() {
  return (
    <Surface aria-labelledby="route-heading" emphasis="raised">
      <p className="section-label">Reserved B0 workflow</p>
      <h2 id="route-heading" tabIndex={-1}>
        Events
      </h2>
      <p>
        The canonical five-stage route is present so navigation identity stays stable. Event cohorts, exact dietary readiness, and least-disclosure briefs remain compiled out until their owning B0 slices provide typed application services.
      </p>
      <ol aria-label="Events stages" className="stage-list">
        {EVENT_STAGES.map((stage) => (
          <li key={stage}>{stageLabels[stage]}</li>
        ))}
      </ol>
    </Surface>
  );
}
