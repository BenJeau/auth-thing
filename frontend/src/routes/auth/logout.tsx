import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { toast } from "sonner";
import { useEffect } from "react";

import { userAtom } from "@/atoms/auth";
import { store } from "@/atoms";
import { Trans } from "@/components";
import { queryClient } from "@/api";

const Logout: React.FC = () => {
  const navigate = useNavigate();

  useEffect(() => {
    queryClient.clear();
    store.set(userAtom, undefined);
    setTimeout(() => {
      toast(<Trans id="logged.out" />, {
        description: <Trans id="logged.out.description" />,
        id: "logout",
      });
      navigate({ to: "/auth/login", replace: true });
    }, 50);
  }, [navigate]);

  return null;
};

export const Route = createFileRoute("/auth/logout")({
  component: Logout,
});
