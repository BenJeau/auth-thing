import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/apps/create/")({
  component: () => <div>Hello /apps/create/!</div>,
});
