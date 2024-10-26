import { QueryClient } from "@tanstack/react-query";
import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";

import { BASE_API_URL } from "@/lib/config";
import type { paths } from "@/api/openapi-client";

const fetchClient = createFetchClient<paths>({
  baseUrl: BASE_API_URL,
});

export const api = createClient(fetchClient);
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
    },
  },
});
