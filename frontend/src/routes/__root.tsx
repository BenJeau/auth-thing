import { QueryClient } from "@tanstack/react-query";
import { createRootRouteWithContext } from "@tanstack/react-router";
import { lazy, Suspense } from "react";

import { Layouts } from "@/components";

const TanStackRouterDevtools =
  process.env.NODE_ENV === "production"
    ? () => null
    : lazy(() =>
        import("@tanstack/router-devtools").then((res) => ({
          default: res.TanStackRouterDevtools,
        }))
      );

const RouteComponent = () => (
  <>
    <Layouts.default />
    <Suspense>
      <TanStackRouterDevtools />
    </Suspense>
  </>
);

export const Route = createRootRouteWithContext<{
  queryClient: QueryClient;
}>()({
  component: RouteComponent,
});
