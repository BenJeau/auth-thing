import { createFileRoute } from "@tanstack/react-router";

import { api } from "@/api";
import { EntryCount } from "@/components";

const IndexComponent: React.FC = () => {
  const users = api.useSuspenseQuery("get", "/users");

  return (
    <div>
      <h1 className="text-xl font-bold flex gap-2 items-baseline">
        Users
        <EntryCount count={users.data.length} />
      </h1>
      <ul>
        {users.data.map((user) => (
          <li key={user.id}>{user.name}</li>
        ))}
      </ul>
    </div>
  );
};

export const Route = createFileRoute("/")({
  component: IndexComponent,
  loader: async ({ context: { queryClient } }) =>
    queryClient.ensureQueryData(api.queryOptions("get", "/users")),
});
