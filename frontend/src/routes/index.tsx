import { createFileRoute } from "@tanstack/react-router";
import { api } from "@/api";

const IndexComponent: React.FC = () => {
  const users = api.useSuspenseQuery("get", "/users");
  return <pre>{JSON.stringify(users.data, null, 4)}</pre>;
};

export const Route = createFileRoute("/")({
  component: IndexComponent,
  loader: async ({ context: { queryClient } }) =>
    queryClient.ensureQueryData(api.queryOptions("get", "/users")),
});
