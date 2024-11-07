import createClient from "openapi-react-query";

import type { components } from "@/api/openapi-client";
import { fetchClient } from "@/api/fetch-client";
import { queryClient } from "@/api/query-client";

type models = components["schemas"];

const api = createClient(fetchClient);

export { queryClient, api };
export type { models };
