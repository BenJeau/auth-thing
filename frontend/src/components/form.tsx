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
    <p className="text-sm text-destructive min-h-[20px]">
      {field.state.meta.errors.join(", ")}
    </p>
  );
};
