import { Button } from "@/components/ui/button";
import { useQuery } from "@tanstack/react-query";
import { usersQueryOptions } from "../api/users";

export default function App() {
  const users = useQuery(usersQueryOptions);

  return (
    <div>
      <Button>Click me</Button>
      <pre>{JSON.stringify(users.data, null, 4)}</pre>
    </div>
  );
}
