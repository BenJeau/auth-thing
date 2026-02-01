import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Pencil, RectangleEllipsis, Trash } from "lucide-react";

import { api, queryClient } from "@/api";
import { Layouts, Title } from "@/components";
import { Button } from "@/components/ui/button";
import { beforeLoadAuthenticated } from "@/lib/auth";
import { Badge } from "@/components/ui/badge";

const ApplicationComponent: React.FC = () => {
  const params = Route.useParams();
  const application = api.useSuspenseQuery("get", "/applications/{id}", {
    params: { path: { id: params.id } },
  });

  const deleteApplication = api.useMutation("delete", "/applications/{id}");
  const navigate = useNavigate();

  const handleOnDelete = async () => {
    await deleteApplication.mutateAsync({
      params: { path: { id: params.id } },
    });

    await Promise.all([
      queryClient.invalidateQueries({ queryKey: ["get", "/applications"] }),
    ]);

    navigate({ to: "/" });
  };

  return (
    <Layouts.Container color="emerald" className="gap-6">
      <Title
        title={
          <div className="ms-2 flex flex-col">
            <div className="flex-1">{application.data.name}</div>
            <div className="-mt-1 flex-none text-sm font-normal">
              {application.data.description}
            </div>
          </div>
        }
        back="/"
        extra={
          <div className="flex gap-2">
            <Button variant="outline">
              <Pencil size={16} />
              Edit
            </Button>
            <Button variant="destructive" onClick={handleOnDelete}>
              <Trash size={16} />
              Delete
            </Button>
          </div>
        }
      />
      <div className="flex flex-col gap-4 rounded-lg border border-emerald-600/40 bg-white/20 p-4 shadow-inner">
        <div className="flex items-center justify-between gap-2 font-semibold">
          <div className="flex gap-2">
            <RectangleEllipsis /> Password Authentication
          </div>
          <div className="flex gap-2 text-sm">
            <Badge
              variant={
                application.data.passwordAuthEnabled ? "success" : "destructive"
              }
            >
              {application.data.passwordAuthEnabled ? "Enabled" : "Disabled"}
            </Badge>
            <Badge
              variant={
                application.data.passwordAuthSignupEnabled
                  ? "success"
                  : "destructive"
              }
            >
              {application.data.passwordAuthSignupEnabled
                ? "Signup Enabled"
                : "Signup Disabled"}
            </Badge>
          </div>
        </div>
        <div className="space-y-1">
          <div className="font-medium">Password Requirements:</div>
          <ul className="list-inside list-disc space-y-0.5 pl-2 text-sm">
            <li>
              Length: {application.data.passwordMinLength} to{" "}
              {application.data.passwordMaxLength} characters
            </li>
            <li>
              Minimum lowercase letters: {application.data.passwordMinLowercase}
            </li>
            <li>
              Minimum uppercase letters: {application.data.passwordMinUppercase}
            </li>
            <li>Minimum numbers: {application.data.passwordMinNumber}</li>
            <li>
              Minimum special characters: {application.data.passwordMinSpecial}
            </li>
            <li>
              Must be unique: {application.data.passwordUnique ? "Yes" : "No"}
            </li>
            <li>Minimum strength: {application.data.passwordMinStrength}</li>
          </ul>
        </div>
      </div>
    </Layouts.Container>
  );
};

export const Route = createFileRoute("/apps/$id")({
  component: ApplicationComponent,
  beforeLoad: beforeLoadAuthenticated(),
  loader: async ({ context: { queryClient }, params }) => {
    await queryClient.ensureQueryData(
      api.queryOptions("get", "/applications/{id}", {
        params: {
          path: { id: params.id },
        },
      }),
    );
  },
});
