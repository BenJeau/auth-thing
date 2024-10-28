import { createFileRoute, useNavigate } from "@tanstack/react-router";

import { api, queryClient } from "@/api";
import { Layouts, Title } from "@/components";
import { Button } from "@/components/ui/button";

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
    <Layouts.Container color="emerald">
      <Title
        title={application.data.name}
        back="/"
        extra={
          <Button variant="destructive" onClick={handleOnDelete}>
            Delete
          </Button>
        }
      />
    </Layouts.Container>
  );
};

export const Route = createFileRoute("/apps/$id")({
  component: ApplicationComponent,
  loader: async ({ context: { queryClient }, params }) => {
    await queryClient.ensureQueryData(
      api.queryOptions("get", "/applications/{id}", {
        params: {
          path: { id: params.id },
        },
      })
    );
  },
});
