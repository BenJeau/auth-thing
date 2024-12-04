import { useAtom } from "jotai";
import { createFileRoute, Navigate } from "@tanstack/react-router";
import { toast } from "sonner";

import { userAtom } from "@/atoms/auth";
import { parseJwt } from "@/lib/auth";
import { useTranslation } from "@/i18n";

const SaveUserData: React.FC = () => {
  const [user, setUser] = useAtom(userAtom);
  const { t } = useTranslation();

  const search = new URLSearchParams(location.search);
  const token = search.get("token");
  const next = search.get("next");

  if (user) {
    if (user.emailVerified) {
      return <Navigate to={next ?? "/"} />;
    }
    return <Navigate to="/auth/verify-email" params={{ next }} />;
  }

  let error = null;
  if (token) {
    console.log("has token");
    const claims = parseJwt(token);

    if (claims.email && claims.sub) {
      let initials: string = "";
      if (claims.given_name && claims.family_name) {
        initials = claims.given_name[0] + claims.family_name;
      } else if (claims.name && claims.name[0]) {
        initials = claims.name[0];
        const parts = claims.name.split(" ");
        if (parts.length > 1 && parts[1]) {
          initials += parts[1][0];
        }
      }

      setUser({
        token,
        name: claims.name,
        emailVerified: claims.email_verified ?? false,
        givenName: claims.given_name,
        familyName: claims.family_name,
        email: claims.email,
        id: claims.sub,
        roles: claims.roles,
        initials: initials.toUpperCase(),
        emailCodeCreatedAt: claims.email_code_created_at,
      });

      if (claims.email_verified) {
        return <Navigate to={next ?? "/"} />;
      }
      return <Navigate to="/auth/verify-email" />;
    } else {
      console.log("missing fields in token");
      error = "invalidToken";
    }
  }

  if (error === null) {
    error = "missingToken";
  }

  let description = t("auth.error.token.missing");

  if (error === "disabled") {
    description = t("auth.error.disabled");
  } else if (error === "invalidToken") {
    description = t("auth.error.token.invalid");
  }

  toast(t("auth.error.title"), {
    id: description,
    description,
  });

  console.log("navigating to login", description);

  return <Navigate to="/auth/login" />;
};

export const Route = createFileRoute("/auth/")({
  component: SaveUserData,
});
