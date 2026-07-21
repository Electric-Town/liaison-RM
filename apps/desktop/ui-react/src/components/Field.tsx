import { useId, type InputHTMLAttributes, type ReactNode } from "react";

export interface FieldProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "id"> {
  readonly id?: string;
  readonly label: string;
  readonly hint?: ReactNode;
  readonly error?: string;
}

export function Field({ id, label, hint, error, className = "", ...input }: FieldProps) {
  const generatedId = useId();
  const inputId = id ?? generatedId;
  const hintId = hint === undefined ? undefined : `${inputId}-hint`;
  const errorId = error === undefined ? undefined : `${inputId}-error`;
  const describedBy = [hintId, errorId].filter(Boolean).join(" ") || undefined;

  return (
    <div className={`lrm-field ${className}`.trim()}>
      <label className="lrm-field__label" htmlFor={inputId}>
        {label}
      </label>
      {hint === undefined ? null : (
        <p className="lrm-field__hint" id={hintId}>
          {hint}
        </p>
      )}
      <input
        {...input}
        aria-describedby={describedBy}
        aria-invalid={error === undefined ? undefined : true}
        className="lrm-field__input"
        id={inputId}
      />
      {error === undefined ? null : (
        <p className="lrm-field__error" id={errorId}>
          {error}
        </p>
      )}
    </div>
  );
}
