interface FieldErrorsProps {
  field: {
    state: {
      meta: {
        errors: string[];
      };
    };
  };
}

export const FieldErrors: React.FC<FieldErrorsProps> = ({ field }) => {
  if (!field.state.meta.errors.length) {
    return null;
  }

  return (
    <p className="min-h-[20px] text-sm text-destructive">
      {field.state.meta.errors.join(", ")}
    </p>
  );
};
