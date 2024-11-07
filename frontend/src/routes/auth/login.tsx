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
        <Forms.Login.default
          onSubmit={handleOnSubmit}
          loading={login.isPending}
          error={login.isError}
        />
        <p className="mt-2 text-center text-xs text-muted-foreground">
          <Trans
            id="login.signup.description"
            link={
              <Link
                className="text-primary underline"
                to={"/auth/signup"}
                search={{ next }}
              >
                <Trans id="signup.here" />
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
});
