import { forwardRef, type ButtonHTMLAttributes } from "react";

export type ButtonTone = "primary" | "secondary" | "danger" | "quiet";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  readonly tone?: ButtonTone;
  readonly busy?: boolean;
  readonly busyLabel?: string;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(function Button(
  {
    tone = "secondary",
    busy = false,
    busyLabel = "Working…",
    disabled,
    className = "",
    children,
    type = "button",
    ...rest
  },
  ref,
) {
  const unavailable = disabled === true || busy;
  return (
    <button
      {...rest}
      aria-busy={busy || undefined}
      className={`lrm-button lrm-button--${tone} ${className}`.trim()}
      disabled={unavailable}
      ref={ref}
      type={type}
    >
      {busy ? busyLabel : children}
    </button>
  );
});
