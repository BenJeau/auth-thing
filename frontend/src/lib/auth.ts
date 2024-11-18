import { ParsedLocation, redirect } from "@tanstack/react-router";
import { toast } from "sonner";

import { User, userAtom } from "@/atoms/auth";
import { store } from "@/atoms";

type Claims = {
  iss: string;
  aud: string;
  exp: number;
  iat: number;
  nbf: number;
  sub: number;
  email: string;
  email_verified?: boolean;
  name?: string;
  given_name?: string;
  family_name?: string;
  locale?: string;
  roles: string[];
  provider: string;
  email_code_created_at?: string;
};

export type BeforeLoadFn = (
  roles?: string[]
) => (opts: { location: ParsedLocation; cause?: string }) => void;

export const beforeLoadAuthenticated: BeforeLoadFn =
  (roles) =>
  ({ location, cause }) => {
    const user = store.get(userAtom);

    if (!user) {
      console.log("HOOK - no user");
      throw redirect({
        to: "/auth/login",
        search: {
          next: location.pathname !== "/" ? location.pathname : undefined,
        },
      });
    }

    if (!user.emailVerified) {
      console.log("HOOK - no email verified", user.emailVerified);
      throw redirect({ to: "/auth/verify-email" });
    }

    if (roles && roles.length > 0) {
      const hasRoles = userHasRoles(user, roles);

      if (!hasRoles) {
        if (cause !== "preload") {
          const missingRoles = roles
            .filter((role) => !user.roles.includes(role))
            .join(", ");
          toast.error(
            `You don't have the required roles to access the page ${window.location.pathname}`,
            {
              id: `no-roles${missingRoles}`,
              description: `Missing roles: ${missingRoles}`,
            }
          );
        }

        throw redirect({ to: "/", replace: true });
      }
    }
  };

export function parseJwt(token: string): Claims {
  const tokenParts = token.split(".");

  if (tokenParts.length !== 3 || !tokenParts[1]) {
    throw new Error("Invalid token");
  }

  const base64 = tokenParts[1].replace(/-/g, "+").replace(/_/g, "/");
  const jsonPayload = decodeURIComponent(
    window
      .atob(base64)
      .split("")
      .map(function (c) {
        return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
      })
      .join("")
  );

  return JSON.parse(jsonPayload) as Claims;
}

export const userHasRoles = (user?: User, roles?: string[]) =>
  !roles || roles.every((role) => user?.roles.includes(role));

export const userHasAnyRoles = (user?: User, roles?: string[]) =>
  !roles || roles.some((role) => user?.roles.includes(role));
