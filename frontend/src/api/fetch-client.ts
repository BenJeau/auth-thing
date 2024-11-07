import createFetchClient from "openapi-fetch";
import { notFound } from "@tanstack/react-router";
import { toast } from "sonner";

import { BASE_API_URL } from "@/lib/config";
import type { paths } from "@/api/openapi-client";
import { store } from "@/atoms";
import { userAtom } from "@/atoms/auth";
import { languageAtom } from "@/atoms/language";
import { languages } from "@/i18n/lang";
import { router } from "@/navigation";
import { anySignal } from "@/lib/utils";

export const fetchClient = createFetchClient<paths>({
  baseUrl: BASE_API_URL,
  fetch: async (request) => {
    const language = store.get(languageAtom);
    const token = store.get(userAtom)?.token;

    if (token) {
      request.headers.set("Authorization", `Bearer ${token}`);
    }

    const signals = [AbortSignal.timeout(30000)];
    if (request.signal) signals.push(request.signal);

    const response = await fetch(request, {
      signal: anySignal(signals),
      keepalive: true,
    });

    if (response.status === 404) {
      throw notFound();
    }

    if (!response.ok) {
      const text = await response.text();

      const message = `${response.status.toString()} - ${response.statusText}${
        text && `: ${text}`
      }`;

      const parsedUrl = new URL(request.url);

      if (response.status === 401 && !parsedUrl.pathname.startsWith("/auth")) {
        store.set(userAtom, undefined);
        toast(
          languages[language].file["authentication.expired.title"] as string,
          {
            description: languages[language].file[
              "authentication.expired.description"
            ] as string,
            id: "expired.auth",
          }
        );
        router.navigate({
          to: "/auth/login",
          search: {
            next: location.pathname,
          },
        });
      } else {
        toast.error(`${response.status.toString()} - ${response.statusText}`, {
          description: text || undefined,
          id: `${response.status.toString()} - ${response.statusText} - ${text}`,
        });
      }

      throw new Error(message);
    }

    return response;
  },
});
