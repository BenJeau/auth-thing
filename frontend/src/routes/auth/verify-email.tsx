import { useAtom } from "jotai";
import { createFileRoute, Navigate, useNavigate } from "@tanstack/react-router";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { userAtom } from "@/atoms/auth";
import { api } from "@/api";
import { Trans, Forms } from "@/components";
import { useTranslation } from "@/i18n";
import { useResendTimer } from "@/hooks/use-resend-timer";

const VerifyEmail: React.FC = () => {
  const [user, setUser] = useAtom(userAtom);
  const navigate = useNavigate();
  const { t } = useTranslation();

  const search = new URLSearchParams(location.search);
  const token = search.get("token");
  const next = search.get("next");

  const verifyEmailMutation = api.useMutation(
    "post",
    "/auth/applications/{slug}/verify/email",
  );

  const resendMutation = api.useMutation(
    "post",
    "/auth/applications/{slug}/verify/email/resend",
  );

  const { canResend, timeLeft, setCanResend } = useResendTimer(
    user?.emailCodeCreatedAt,
  );

  const handleSubmit = async (
    values: Forms.TokenVerification.FormSchema,
  ): Promise<boolean> => {
    const response = await verifyEmailMutation.mutateAsync({
      params: {
        path: { slug: "example" },
        query: { token: values.token },
      },
    });

    if (response.success) {
      setUser((prev) => ({ ...prev!, emailVerified: true }));
      toast.success(t("auth.verify.success"));
      await navigate({ to: next ?? "/" });
      return false;
    }

    return true;
  };

  const handleResend = async () => {
    try {
      await resendMutation.mutateAsync({
        params: { path: { slug: "example" } },
      });
      toast.success(t("auth.verify.resend.success"));
      setCanResend(false);
    } catch {
      toast.error(t("auth.verify.resend.error"), { id: "resend-error" });
    }
  };

  if (user?.emailVerified) {
    return <Navigate to={next ?? "/"} />;
  }

  return (
    <div className="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-3xl font-bold tracking-tight">
          <Trans id="auth.verify.email.title" />
        </h2>
        <p className="mt-2 text-center text-sm">
          <Trans id="auth.verify.email.description" />
        </p>

        <div className="mt-8">
          <Forms.TokenVerification.default
            defaultToken={token}
            onSubmit={handleSubmit}
            loading={verifyEmailMutation.isPending}
            withSeparator={false}
          />

          <div className="mt-4 text-center">
            {canResend ? (
              <Button
                variant="link"
                onClick={handleResend}
                disabled={resendMutation.isPending}
              >
                <Trans id="auth.verify.resend.button" />
              </Button>
            ) : timeLeft > 0 ? (
              <p className="text-sm text-gray-500">
                <Trans id="auth.verify.resend.countdown" seconds={timeLeft} />
              </p>
            ) : null}
          </div>
        </div>
      </div>
    </div>
  );
};

export const Route = createFileRoute("/auth/verify-email")({
  component: VerifyEmail,
});
