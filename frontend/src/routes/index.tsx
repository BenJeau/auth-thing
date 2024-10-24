import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";

import { Button } from "@/components/ui/button";
import { usersQueryOptions } from "@/api/users";

const IndexComponent: React.FC = () => {
  const users = useSuspenseQuery(usersQueryOptions);

  return (
    <div>
      <Button>Click me</Button>
      <pre>{JSON.stringify(users.data, null, 4)}</pre>
    </div>
  );
};

export const Route = createFileRoute("/")({
  component: IndexComponent,
  loader: async ({ context: { queryClient } }) =>
    await queryClient.ensureQueryData(usersQueryOptions),
});
