import {
  Link,
  Navigate,
  createFileRoute,
  redirect,
  useNavigate,
} from "@tanstack/react-router";
import { toast } from "sonner";

import { api } from "@/api";
import { Forms, Trans } from "@/components";
import { store } from "@/atoms";
import { userAtom } from "@/atoms/auth";
import { useTranslation } from "@/i18n";

const Signup: React.FC = () => {
  const { next } = Route.useSearch();
  const signup = api.useMutation("post", "/auth/applications/{slug}/signup");
  const navigate = useNavigate();
  const { t } = useTranslation();
  const config = api.useSuspenseQuery(
    "get",
    "/auth/applications/{slug}/config",
    {
      params: { path: { slug: "example" } },
    },
  );

  if (!config.data.password.signupEnabled) {
    toast.info(t("signup.disabled.toast.title"), {
      description: t("signup.disabled.toast.description"),
      id: "signup.disabled",
    });
    return <Navigate to="/auth/login" replace />;
  }

  const handleOnSubmit = async (data: Forms.Signup.FormSchema) => {
    await signup.mutateAsync({
      body: data,
      params: {
        path: {
          slug: "example",
        },
      },
    });

    toast(<Trans id="user.created.title" />, {
      description: <Trans id="user.created.description" />,
    });

    await navigate({
      to: "/auth/login",
      search: {
        next,
      },
    });
  };

  return (
    <>
      <div className="flex flex-col space-y-2 text-center">
        <h1 className="text-2xl font-semibold tracking-tight">
          <Trans id="signup" />
        </h1>
        <p className="text-sm text-muted-foreground">
          <Trans id="signup.description" />
        </p>
      </div>
      <div className="grid gap-2">
        <Forms.Signup.default
          onSubmit={handleOnSubmit}
          loading={signup.isPending}
        />
        <p className="mt-2 text-center text-xs text-muted-foreground">
          <Trans
            id="signup.login.description"
            link={
              <Link
                className="text-primary underline"
                to="/auth/login"
                search={{ next }}
              >
                <Trans id="login.here" />
              </Link>
            }
          />
        </p>
      </div>
    </>
  );
};

interface SearchParams {
  next?: string;
}

export const Route = createFileRoute("/auth/signup")({
  component: Signup,
  beforeLoad: () => {
    const user = store.get(userAtom);

    if (user) {
      throw redirect({ to: "/" });
    }
  },
  validateSearch: ({ next }: SearchParams): SearchParams => ({
    next: next && next !== "/" ? next : undefined,
  }),
  loader: async ({ context: { queryClient } }) => {
    await queryClient.ensureQueryData(
      api.queryOptions("get", "/auth/applications/{slug}/config", {
        params: {
          path: {
            slug: "example",
          },
        },
      }),
    );
  },
});
