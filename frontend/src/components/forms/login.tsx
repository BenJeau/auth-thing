import * as v from "valibot";
import { CircleAlert, Loader2, LogIn } from "lucide-react";
import { useEffect, useMemo, useState } from "react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Form, AutoAnimate, Trans } from "@/components";
import { useTranslation } from "@/i18n";
import { useForm } from "@tanstack/react-form";
import { valibotValidator } from "@tanstack/valibot-form-adapter";

const formSchema = v.object({
  email: v.pipe(v.string(), v.email()),
  password: v.string(),
});

export type FormSchema = v.InferOutput<typeof formSchema>;

interface Props {
  onSubmit: (values: FormSchema) => Promise<void>;
  loading: boolean;
  error: boolean;
}

const Login: React.FC<Props> = ({ onSubmit, loading, error }) => {
  const { t } = useTranslation();
  const form = useForm({
    validatorAdapter: valibotValidator(),
    validators: {
      onChange: formSchema,
    },
    defaultValues: { email: "", password: "" },
    onSubmit: async ({ value }) => {
      await onSubmit(value);
    },
  });

  const [errorValues, setErrorValues] = useState<FormSchema>(form.state.values);

  useEffect(() => {
    if (error) {
      setErrorValues(form.state.values);
    }
  }, [error, form.state.values]);

  const isError = useMemo(() => {
    return (
      error &&
      form.state.values.email === errorValues.email &&
      form.state.values.password === errorValues.password
    );
  }, [error, errorValues, form.state.values]);

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        e.stopPropagation();
        form.handleSubmit();
      }}
      className="flex h-full flex-col gap-2"
    >
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
        <Button
          className="w-full gap-2"
          type="submit"
          variant={isError ? "destructive" : undefined}
          disabled={loading}
        >
          <AutoAnimate>
            {loading && <Loader2 size={16} className="animate-spin" />}
            {isError && <CircleAlert size={16} />}
            {!loading && !isError && <LogIn size={16} />}
          </AutoAnimate>
          <Trans id="authenticate" />
        </Button>
      </AutoAnimate>
    </form>
  );
};

export default Login;
