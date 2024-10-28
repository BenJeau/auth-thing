import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { AnimatePresence, motion } from "framer-motion";

import { api, queryClient } from "@/api";
import { Title } from "@/components";
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
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: 10 }}
        transition={{ duration: 0.5 }}
        className="flex gap-8 flex-col bg-emerald-100 dark:bg-emerald-950 rounded-t-2xl shadow-lg border-2 border-b-0 border-emerald-400 dark:border-emerald-900 flex-1 relative"
      >
        <div className="p-5 flex gap-4 flex-col min-h-[300px] sticky top-0">
          <Title
            title={application.data.name}
            back="/"
            extra={
              <Button variant="destructive" onClick={handleOnDelete}>
                Delete
              </Button>
            }
          />
        </div>
      </motion.div>
    </AnimatePresence>
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
