import { useAtom } from "jotai";
import { createFileRoute, Navigate, useNavigate } from "@tanstack/react-router";
import { toast } from "sonner";

import { userAtom } from "@/atoms/auth";
import { api } from "@/api";
import { Trans, Forms } from "@/components";
import { useTranslation } from "@/i18n";

const VerifyEmail = () => {
  const [user, setUser] = useAtom(userAtom);
  const navigate = useNavigate();
  const { t } = useTranslation();

  const search = new URLSearchParams(location.search);
  const token = search.get("token");
  const next = search.get("next");

  const verifyEmailMutation = api.useMutation(
    "post",
    "/auth/applications/{slug}/verify/email"
  );

  const handleSubmit = async (
    values: Forms.TokenVerification.FormSchema
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

  console.log("verify page - verified?", user?.emailVerified);
  if (user?.emailVerified) {
    return <Navigate to={next ?? "/"} />;
  }

  return (
    <div className="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-3xl font-bold tracking-tight">
          <Trans id="auth.verify.email.title" />
        </h2>
        <p className="mt-2 text-center text-sm text-gray-600">
          <Trans id="auth.verify.email.description" />
        </p>

        <div className="mt-8">
          <Forms.TokenVerification.default
            defaultToken={token}
            onSubmit={handleSubmit}
            loading={verifyEmailMutation.isPending}
            withSeparator={false}
          />
        </div>
      </div>
    </div>
  );
};

export const Route = createFileRoute("/auth/verify-email")({
  component: VerifyEmail,
});
