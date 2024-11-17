import { Loader } from "lucide-react";
import { useForm } from "@tanstack/react-form";
import * as v from "valibot";
import { valibotValidator } from "@tanstack/valibot-form-adapter";

import {
  InputOTP,
  InputOTPGroup,
  InputOTPSlot,
  InputOTPSeparator,
} from "@/components/ui/input-otp";
import { AutoAnimate, Form, Trans } from "@/components";
import { useState } from "react";

const formSchema = v.object({
  token: v.pipe(v.string(), v.length(8)),
});

export type FormSchema = v.InferOutput<typeof formSchema>;

interface Props {
  defaultToken?: string | null;
  onSubmit: (values: FormSchema) => Promise<boolean>;
  loading?: boolean;
  withSeparator?: boolean;
}

const TokenVerification: React.FC<Props> = ({
  defaultToken,
  onSubmit,
  loading,
  withSeparator = true,
}) => {
  const [isWrongCode, setIsWrongCode] = useState(false);

  const form = useForm({
    validatorAdapter: valibotValidator(),
    validators: {
      onChange: formSchema,
    },
    defaultValues: { token: defaultToken || "" },
    onSubmit: async ({ value }) => {
      console.log("submitting", value);
      const error = await onSubmit(value);

      setIsWrongCode(error);
    },
  });

  const secondSetOfSlots = (
    <>
      <InputOTPSlot index={4} />
      <InputOTPSlot index={5} />
      <InputOTPSlot index={6} />
      <InputOTPSlot index={7} />
    </>
  );

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        e.stopPropagation();
        form.handleSubmit();
      }}
      className="flex h-full flex-col items-center gap-4 relative"
    >
      <form.Field name="token">
        {(field) => (
          <div className="grid gap-2">
            <InputOTP
              maxLength={8}
              disabled={loading}
              value={field.state.value}
              className={loading ? "opacity-50" : ""}
              onChange={(value) => {
                field.handleChange(value);
                if (isWrongCode) {
                  setIsWrongCode(false);
                }
                if (value.length === 8) {
                  form.handleSubmit();
                }
              }}
            >
              <InputOTPGroup>
                <InputOTPSlot index={0} />
                <InputOTPSlot index={1} />
                <InputOTPSlot index={2} />
                <InputOTPSlot index={3} />
                {!withSeparator && secondSetOfSlots}
              </InputOTPGroup>
              {withSeparator && (
                <>
                  <InputOTPSeparator />
                  <InputOTPGroup>{secondSetOfSlots}</InputOTPGroup>
                </>
              )}
            </InputOTP>
            <Form.FieldErrors
              field={{
                state: {
                  meta: {
                    errors: isWrongCode ? ["Wrong verification code"] : [],
                  },
                },
              }}
            />
          </div>
        )}
      </form.Field>
      {loading && (
        <AutoAnimate className="flex items-center gap-2 absolute top-0 left-0 right-0 bottom-0 backdrop-blur-sm -m-4">
          <Loader size={16} className="animate-spin" />
          <Trans id="auth.verify.verifying" />
        </AutoAnimate>
      )}
    </form>
  );
};

export default TokenVerification;
