import { forwardRef, type HTMLAttributes } from "react";

export type SurfaceEmphasis = "default" | "raised" | "quiet";

export interface SurfaceProps extends HTMLAttributes<HTMLElement> {
  readonly as?: "section" | "article" | "div";
  readonly emphasis?: SurfaceEmphasis;
}

export const Surface = forwardRef<HTMLElement, SurfaceProps>(function Surface(
  {
    as: Element = "section",
    emphasis = "default",
    className = "",
    children,
    ...rest
  },
  ref,
) {
  return (
    <Element
      {...rest}
      className={`lrm-surface lrm-surface--${emphasis} ${className}`.trim()}
      ref={ref as never}
    >
      {children}
    </Element>
  );
});
