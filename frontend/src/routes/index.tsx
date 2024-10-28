import { createFileRoute, Link } from "@tanstack/react-router";
import { Filter, Plus } from "lucide-react";
import { ColumnDef } from "@tanstack/react-table";
import { AnimatePresence, motion } from "framer-motion";

import { api, models } from "@/api";
import { Title } from "@/components";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

import {
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { cn } from "../lib/utils";

interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
  className?: string;
}

export function DataTable<TData, TValue>({
  columns,
  data,
  className,
}: DataTableProps<TData, TValue>) {
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <div
      className={cn(
        "rounded-xl border shadow-inner dark:bg-black/50 bg-white/50",
        className
      )}
    >
      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow
                key={row.id}
                data-state={row.getIsSelected() && "selected"}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell colSpan={columns.length} className="h-24 text-center">
                No results.
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </div>
  );
}

const userTableColumnDef: ColumnDef<models["User"]>[] = [
  {
    accessorKey: "id",
    header: "ID",
  },
  {
    accessorKey: "name",
    header: "Name",
  },
  {
    accessorKey: "email",
    header: "Email",
  },
];

const ApplicationCard: React.FC<{ application: models["Application"] }> = ({
  application,
}) => (
  <Link
    to="/apps/$id"
    params={{ id: application.id.toString() }}
    className="flex flex-col border border-emerald-500 p-4 rounded-lg shadow-inner  bg-white/50 dark:bg-black/30 dark:hover:bg-black/60 hover:bg-white transition-all cursor-pointer"
  >
    <h2 className="font-bold">{application.name}</h2>
    <p>{application.description || "-"}</p>
  </Link>
);

const IndexComponent: React.FC = () => {
  const users = api.useSuspenseQuery("get", "/users");
  const applications = api.useSuspenseQuery("get", "/applications");

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
            title="Applications"
            count={applications.data.length}
            extra={
              <div className="flex gap-2">
                <Input type="text" placeholder="Filter" />
                <Button className="bg-emerald-400">
                  <Filter />
                </Button>
              </div>
            }
          />
          <div className="grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 grid gap-2">
            {applications.data.map((application) => (
              <ApplicationCard key={application.id} application={application} />
            ))}
            <Link
              to="/apps/create"
              className="items-center border border-emerald-400 border-dashed p-4 rounded-lg bg-white/50 dark:bg-black/30 dark:hover:bg-black/60 hover:bg-white transition-all cursor-pointer flex gap-2 shadow-inner"
            >
              <Plus />
              <h2 className="text-md font-bold">Create a new application</h2>
            </Link>
          </div>
        </div>
        <AnimatePresence>
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 10 }}
            transition={{ duration: 0.5 }}
            className="flex gap-8 flex-col bg-cyan-100 -mx-[8px] dark:bg-cyan-950 rounded-t-2xl shadow-xl border-2 border-b-0 border-cyan-400 dark:border-cyan-900 flex-1 z-10 relative"
          >
            <div className="flex gap-4 flex-col p-5 sticky  min-h-[300px] top-0">
              <Title
                title="Users"
                count={users.data.length}
                extra={
                  <div className="flex gap-2">
                    <Input type="text" placeholder="Filter" />
                    <Button>
                      <Filter />
                    </Button>
                  </div>
                }
              />
              <DataTable
                columns={userTableColumnDef}
                data={users.data}
                className="border-cyan-400"
              />
            </div>
            <AnimatePresence>
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 10 }}
                transition={{ duration: 0.5 }}
                className="bg-fuchsia-100 -mx-[8px] dark:bg-fuchsia-950 rounded-t-2xl shadow-2xl border-2 border-b-0 border-fuchsia-400 dark:border-fuchsia-900 flex-1 min-h-[5000px] z-20 relative"
              >
                <div className="flex gap-4 flex-col  p-5 sticky top-0">
                  <Title
                    title="Authentication Providers"
                    extra={
                      <div className="flex gap-2">
                        <Input type="text" placeholder="Filter" />
                        <Button className="bg-fuchsia-400">
                          <Filter />
                        </Button>
                      </div>
                    }
                  />
                  <DataTable
                    columns={[]}
                    data={[]}
                    className="border-fuchsia-400"
                  />
                </div>
              </motion.div>
            </AnimatePresence>
          </motion.div>
        </AnimatePresence>
      </motion.div>
    </AnimatePresence>
  );
};

export const Route = createFileRoute("/")({
  component: IndexComponent,
  loader: async ({ context: { queryClient } }) =>
    Promise.all([
      queryClient.ensureQueryData(api.queryOptions("get", "/users")),
      queryClient.ensureQueryData(api.queryOptions("get", "/applications")),
    ]),
});
