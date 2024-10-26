import { createFileRoute } from "@tanstack/react-router";
import { Filter } from "lucide-react";

import { api } from "@/api";
import { EntryCount } from "@/components";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

const IndexComponent: React.FC = () => {
  const users = api.useSuspenseQuery("get", "/users");

  return (
    <>
      <div className="flex justify-between gap-2 flex-wrap bg-cyan-100 dark:bg-cyan-950 rounded-t-2xl p-6 shadow-inner border border-b-0 border-cyan-400 dark:border-cyan-900 flex-1">
        <h1 className="text-xl font-bold flex gap-2 items-baseline">
          Users
          <EntryCount count={users.data.length} />
        </h1>
        <div className="flex gap-2">
          <Input type="text" placeholder="Filter" />
          <Button>
            <Filter />
          </Button>
        </div>
      </div>
      <ul>
        {users.data.map((user) => (
          <li key={user.id}>{user.name}</li>
        ))}
      </ul>
    </>
  );
};

export const Route = createFileRoute("/")({
  component: IndexComponent,
  loader: async ({ context: { queryClient } }) =>
    queryClient.ensureQueryData(api.queryOptions("get", "/users")),
});
