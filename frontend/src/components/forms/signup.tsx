import * as v from "valibot";
import { Loader2, LogIn } from "lucide-react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { AutoAnimate, Form, Trans } from "@/components";
import { useTranslation } from "@/i18n";
import { useForm } from "@tanstack/react-form";
import { valibotValidator } from "@tanstack/valibot-form-adapter";

const formSchema = v.object({
  name: v.string(),
  email: v.pipe(v.string(), v.email()),
  password: v.string(),
});

export type FormSchema = v.InferOutput<typeof formSchema>;

interface Props {
  onSubmit: (values: FormSchema) => Promise<void>;
  loading: boolean;
}

const Signup: React.FC<Props> = ({ onSubmit, loading }) => {
  const { t } = useTranslation();
  const form = useForm({
    validatorAdapter: valibotValidator(),
    validators: {
      onChange: formSchema,
    },
    onSubmit: async ({ value }) => {
      await onSubmit(value);
    },
    defaultValues: { name: "", email: "", password: "" },
  });

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        e.stopPropagation();
        form.handleSubmit();
      }}
      className="flex h-full flex-col gap-2"
    >
      <form.Field name="name">
        {(field) => (
          <div className="grid gap-2">
            <Input
              placeholder={t("name").toLowerCase()}
              name={field.name}
              value={field.state.value}
              onChange={(e) => field.handleChange(e.target.value)}
            />
            <Form.FieldErrors field={field} />
          </div>
        )}
      </form.Field>
      <form.Field name="email">
        {(field) => (
          <div className="grid gap-2">
            <Input
              placeholder={t("email").toLowerCase()}
              name={field.name}
              value={field.state.value}
              onChange={(e) => field.handleChange(e.target.value)}
            />
            <Form.FieldErrors field={field} />
          </div>
        )}
      </form.Field>
      <form.Field name="password">
        {(field) => (
          <div className="grid gap-2">
            <Input
              placeholder={t("password").toLowerCase()}
              type="password"
              name={field.name}
              value={field.state.value}
              onChange={(e) => field.handleChange(e.target.value)}
            />
            <Form.FieldErrors field={field} />
          </div>
        )}
      </form.Field>
      <AutoAnimate>
        <Button className="w-full gap-2" type="submit" disabled={loading}>
          <AutoAnimate>
            {loading && <Loader2 size={16} className="animate-spin" />}
            {!loading && <LogIn size={16} />}
          </AutoAnimate>
          <Trans id="create.account" />
        </Button>
      </AutoAnimate>
    </form>
  );
};

export default Signup;
