import { queryOptions } from "@tanstack/react-query";

export const usersQueryOptions = queryOptions({
  queryKey: ["users"],
  queryFn: async () => {
    const response = await fetch("https://jsonplaceholder.typicode.com/users");
    const data = await response.json();
    return data;
  },
});
