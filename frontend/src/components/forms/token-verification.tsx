import { useState } from "react";
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
    >
      <form.Field name="token">
        {(field) => (
          <div className="flex flex-col items-center gap-2">
            <div className="relative">
              <InputOTP
                maxLength={8}
                disabled={loading}
                value={field.state.value}
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
              {loading && (
                <AutoAnimate className="flex items-center gap-2 text-sm font-semibold absolute top-0 left-0 right-0 bottom-0 backdrop-blur-sm -m-4 justify-center">
                  <Loader
                    size={20}
                    strokeWidth={2.5}
                    className="animate-spin"
                  />
                  <Trans id="auth.verify.verifying" />
                </AutoAnimate>
              )}
            </div>
            <Form.FieldErrors
              field={{
                state: {
                  meta: {
                    errors: isWrongCode ? ["Wrong verification code"] : [""],
                  },
                },
              }}
            />
          </div>
        )}
      </form.Field>
    </form>
  );
};

export default TokenVerification;
