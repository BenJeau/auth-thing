import {
  Link,
  createFileRoute,
  redirect,
  useNavigate,
} from "@tanstack/react-router";

import { Forms, Trans } from "@/components";
import { store } from "@/atoms";
import { userAtom } from "@/atoms/auth";
import { api } from "@/api";

const Login: React.FC = () => {
  const { next } = Route.useSearch();

  const config = api.useSuspenseQuery(
    "get",
    "/auth/applications/{slug}/config",
    {
      params: { path: { slug: "example" } },
    },
  );

  // const query = next ? `?next=${next}` : "";

  const login = api.useMutation("post", "/auth/applications/{slug}/login");
  const navigate = useNavigate();

  const handleOnSubmit = async (data: Forms.Login.FormSchema) => {
    const { jwtToken } = await login.mutateAsync({
      body: data,
      params: {
        path: {
          slug: "example",
        },
      },
    });

    await navigate({
      to: "/auth",
      search: {
        token: jwtToken,
        next: next,
      },
    });
  };

  const googleEnabled = false;
  const microsoftEnabled = false;

  return (
    <>
      <div className="flex flex-col space-y-2 text-center">
        <h1 className="text-2xl font-semibold tracking-tight">
          <Trans id="login" />
        </h1>
        <p className="text-sm text-muted-foreground">
          <Trans id="authentication.description" />
        </p>
      </div>
      <div className="grid gap-2">
        {/* {googleEnabled && (
          <Button type="button" className="gap-2" asChild variant="outline">
            <a href={`${BASE_API_URL}/auth/openid/google${query}`}>
              <Icons.Google />
              Google
            </a>
          </Button>
        )}
        {microsoftEnabled && (
          <Button type="button" className="gap-2" asChild variant="outline">
            <a href={`${BASE_API_URL}/auth/openid/microsoft${query}`}>
              <Icons.Microsoft />
              Microsoft
            </a>
          </Button>
        )} */}
        {(microsoftEnabled ?? googleEnabled) && (
          <div className="relative my-8 flex items-center justify-center">
            <hr className="flex-1" />
            <div className="absolute self-center bg-background px-6 text-center text-xs italic text-muted-foreground">
              <Trans id="login.alternative.description" />
            </div>
          </div>
        )}
        <div className="relative flex">
          <div className="flex-1 blur-sm">
            <Forms.Login.default
              onSubmit={handleOnSubmit}
              loading={login.isPending}
              error={login.isError}
              disabled={!config.data.password.enabled}
            />
          </div>
          <p className="absolute bottom-0 left-0 right-0 top-0 flex flex-1 items-center justify-center text-center text-sm font-medium italic">
            Password login is currently disabled
          </p>
        </div>
        <p className="mt-2 text-center text-xs text-muted-foreground">
          <Trans
            id={
              config.data.password.signupEnabled
                ? "login.signup.description"
                : "login.signup.disabled.description"
            }
            link={
              <Link
                className="text-primary"
                to={"/auth/signup"}
                search={{ next }}
              >
                <Trans id="signup.here" case="sentence" />
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

export const Route = createFileRoute("/auth/login")({
  component: Login,
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
