import { FieldApi } from "@tanstack/react-form";

export const FieldErrors: React.FC<{
  field: FieldApi<any, any, any, any, any>;
}> = ({ field }) => {
  if (!field.state.meta.errors.length) {
    return null;
  }

  return (
    <p className="text-sm text-destructive min-h-[20px]">
      {field.state.meta.errors.join(", ")}
    </p>
  );
};
