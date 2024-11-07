import { QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { RouterProvider } from "@tanstack/react-router";
import { Provider } from "jotai";

import { Toaster } from "@/components/ui/sonner";
import { store } from "@/atoms";
import { queryClient } from "@/api/query-client";
import { router } from "@/navigation";

const Providers: React.FC = () => (
  <QueryClientProvider client={queryClient}>
    <Provider store={store}>
      <RouterProvider router={router} />
    </Provider>
    <ReactQueryDevtools initialIsOpen={false} />
    <Toaster />
  </QueryClientProvider>
);

export default Providers;
