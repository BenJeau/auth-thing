import { useAtom } from "jotai";
import { createFileRoute, Navigate, useNavigate } from "@tanstack/react-router";

import { userAtom } from "@/atoms/auth";
import { api } from "@/api";
import { Trans, Forms } from "@/components";
import { Container } from "@/components/layouts";

const VerifyOTP = () => {
  const [user] = useAtom(userAtom);
  const navigate = useNavigate();

  const verifyOTPMutation = api.useMutation(
    "post",
    "/auth/applications/{slug}/verify/otp"
  );

  const handleSubmit = async (values: Forms.TokenVerification.FormSchema) => {
    const response = await verifyOTPMutation.mutateAsync({
      params: {
        path: { slug: "example" },
        query: { otp: values.token },
      },
    });

    if (response.redirect) {
      navigate({ to: response.redirect });
    } else {
      navigate({ to: "/" });
    }
  };

  if (user) {
    return <Navigate to="/" />;
  }

  return (
    <Container>
      <div className="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
        <div className="sm:mx-auto sm:w-full sm:max-w-md">
          <h2 className="mt-6 text-center text-3xl font-bold tracking-tight">
            <Trans id="auth.otp.title" />
          </h2>
          <p className="mt-2 text-center text-sm text-gray-600">
            <Trans id="auth.otp.description" />
          </p>
        </div>

        <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
          <div className="bg-white px-4 py-8 shadow sm:rounded-lg sm:px-10">
            <Forms.TokenVerification.default
              onSubmit={handleSubmit}
              loading={verifyOTPMutation.isPending}
            />
          </div>
        </div>
      </div>
    </Container>
  );
};

export const Route = createFileRoute("/auth/verify-otp")({
  component: VerifyOTP,
});
