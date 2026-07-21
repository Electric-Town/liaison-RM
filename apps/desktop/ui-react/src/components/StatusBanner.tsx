import type { ReactNode } from "react";
import type { OperationPhase } from "../application-contract";

export interface StatusBannerProps {
  readonly phase: OperationPhase;
  readonly title: string;
  readonly children: ReactNode;
  readonly recovery?: ReactNode;
  readonly receiptId?: string;
}

const assertivePhases = new Set<OperationPhase>(["conflict", "failed"]);

export function StatusBanner({
  phase,
  title,
  children,
  recovery,
  receiptId,
}: StatusBannerProps) {
  const assertive = assertivePhases.has(phase);
  return (
    <section
      aria-atomic="true"
      aria-labelledby={`operation-${phase}-title`}
      className={`lrm-status lrm-status--${phase}`}
      role={assertive ? "alert" : "status"}
    >
      <h3 id={`operation-${phase}-title`}>{title}</h3>
      <div>{children}</div>
      {recovery === undefined ? null : (
        <div className="lrm-status__recovery">
          <strong>Recovery:</strong> {recovery}
        </div>
      )}
      {receiptId === undefined ? null : (
        <p className="lrm-status__receipt">
          <span>Receipt</span> <code>{receiptId}</code>
        </p>
      )}
    </section>
  );
}
