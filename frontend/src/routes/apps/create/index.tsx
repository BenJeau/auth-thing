import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useForm } from "@tanstack/react-form";
import { zodValidator } from "@tanstack/zod-form-adapter";
import { z } from "zod";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Form, Layouts, Title } from "@/components";
import { api, queryClient } from "@/api";

const formSchema = z.object({
  name: z.string().min(1),
  description: z.string(),
  website: z.string(),
  icon: z.string(),
});

const IndexComponent: React.FC = () => {
  const createApplication = api.useMutation("post", "/applications");
  const navigate = useNavigate();

  const form = useForm({
    defaultValues: { name: "", description: "", website: "", icon: "" },
    validatorAdapter: zodValidator(),
    validators: {
      onChange: formSchema,
    },
    onSubmit: async ({ value }) => {
      const id = await createApplication.mutateAsync({
        body: value,
      });

      await Promise.all([
        queryClient.invalidateQueries({ queryKey: ["get", "/applications"] }),
      ]);

      navigate({ to: "/apps/$id", params: { id } });
    },
  });

  return (
    <Layouts.Container color="emerald">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
        className="flex flex-col gap-4"
      >
        <Title title="Create a new application" back="/" />
        <form.Field name="name">
          {(field) => (
            <div className="grid gap-2">
              <Label htmlFor={field.name}>Name</Label>
              <Input
                placeholder="Name"
                name={field.name}
                value={field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
              />
              <Form.FieldErrors field={field} />
            </div>
          )}
        </form.Field>

        <form.Field name="description">
          {(field) => (
            <div className="grid gap-2">
              <Label htmlFor={field.name}>Description</Label>
              <Textarea
                placeholder="Description"
                name={field.name}
                value={field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
              />
              <Form.FieldErrors field={field} />
            </div>
          )}
        </form.Field>

        <form.Field name="website">
          {(field) => (
            <div className="grid gap-2">
              <Label htmlFor={field.name}>Website</Label>
              <Input
                placeholder="Website"
                name={field.name}
                value={field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
              />
              <Form.FieldErrors field={field} />
            </div>
          )}
        </form.Field>

        <form.Field name="icon">
          {(field) => (
            <div className="grid gap-2">
              <Label htmlFor={field.name}>Icon</Label>
              <Input
                placeholder="Icon"
                name={field.name}
                value={field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
              />
              <Form.FieldErrors field={field} />
            </div>
          )}
        </form.Field>

        <form.Subscribe
          selector={(state) => [state.canSubmit, state.isSubmitting]}
          children={([canSubmit, isSubmitting]) => (
            <Button type="submit" disabled={!canSubmit}>
              {isSubmitting ? "..." : "Submit"}
            </Button>
          )}
        />
      </form>
    </Layouts.Container>
  );
};

export const Route = createFileRoute("/apps/create/")({
  component: IndexComponent,
});
