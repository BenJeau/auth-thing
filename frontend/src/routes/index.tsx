import { createFileRoute, Link } from "@tanstack/react-router";
import { Filter, Plus } from "lucide-react";
import { ColumnDef } from "@tanstack/react-table";

import { api, models } from "@/api";
import { EntryCount } from "@/components";
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

interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
}

export function DataTable<TData, TValue>({
  columns,
  data,
}: DataTableProps<TData, TValue>) {
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <div className="rounded-xl border  shadow-inner bg-white/50">
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
  <div>
    <h2 className="font-bold">{application.name}</h2>
    <p>{application.description}</p>
  </div>
);

const IndexComponent: React.FC = () => {
  const users = api.useSuspenseQuery("get", "/users");
  const applications = api.useSuspenseQuery("get", "/applications");

  return (
    <div className="flex gap-8 flex-col bg-emerald-100 dark:bg-emerald-950/30 rounded-t-2xl pt-5 shadow-inner border border-b-0 border-emerald-400 dark:border-emerald-900 flex-1">
      <div className="px-5 flex gap-4 flex-col">
        <div className="flex justify-between gap-2 flex-wrap">
          <h1 className="text-lg font-bold flex gap-2 items-baseline">
            Applications
            <EntryCount count={applications.data.length} />
          </h1>
          <div className="flex gap-2">
            <Input type="text" placeholder="Filter" />
            <Button className="bg-emerald-400">
              <Filter />
            </Button>
          </div>
        </div>
        {applications.data.map((application) => (
          <ApplicationCard key={application.id} application={application} />
        ))}
        <Link to="/apps/create">
          <div className="border border-emerald-400 border-dashed p-4 rounded-lg bg-white/50 hover:bg-white transition-all cursor-pointer flex gap-2 shadow-inner">
            <Plus />
            <h2 className="text-md font-bold">Create a new application</h2>
          </div>
        </Link>
      </div>
      <div className="flex gap-4 flex-col bg-cyan-100 -mx-[1px] dark:bg-cyan-950/30 rounded-t-2xl p-5 shadow-inner border border-b-0 border-cyan-400 dark:border-cyan-900 flex-1">
        <div className="flex justify-between gap-2 flex-wrap">
          <h1 className="text-lg font-bold flex gap-2 items-baseline">
            Users
            <EntryCount count={users.data.length} />
          </h1>
          <div className="flex gap-2">
            <Input type="text" placeholder="Filter" />
            <Button>
              <Filter />
            </Button>
          </div>
        </div>
        <DataTable columns={userTableColumnDef} data={users.data} />
      </div>
    </div>
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
