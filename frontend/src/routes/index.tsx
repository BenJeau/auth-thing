import { createFileRoute, Link } from "@tanstack/react-router";
import { Filter, Plus } from "lucide-react";
import { ColumnDef } from "@tanstack/react-table";
import { useState } from "react";
import {
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";

import { api, models } from "@/api";
import { Layouts, Title } from "@/components";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { cn } from "@/lib/utils";
import { beforeLoadAuthenticated } from "@/lib/auth";

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
        className,
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
                          header.getContext(),
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
    className="flex flex-col border border-emerald-500 p-4 rounded-xl shadow-inner  bg-white/50 dark:bg-black/30 dark:hover:bg-black/60 hover:bg-white transition-all cursor-pointer"
  >
    <h2 className="font-bold">{application.name}</h2>
    <p>{application.description || "-"}</p>
  </Link>
);

const IndexComponent: React.FC = () => {
  const users = api.useSuspenseQuery("get", "/users");
  const applications = api.useSuspenseQuery("get", "/applications");

  const [filterApplication, setFilterApplication] = useState("");

  const filteredApplications = applications.data.filter((application) =>
    application.name.toLowerCase().includes(filterApplication.toLowerCase()),
  );

  const authenticationProvidersSection = (
    <Layouts.Container color="fuchsia" className="z-20 -mx-[8px]">
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
      <DataTable columns={[]} data={[]} className="border-fuchsia-400" />
    </Layouts.Container>
  );

  const usersSection = (
    <Layouts.Container
      color="cyan"
      className="z-10 -mx-[8px]"
      bottomContent={authenticationProvidersSection}
    >
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
    </Layouts.Container>
  );

  return (
    <Layouts.Container color="emerald" bottomContent={usersSection}>
      <Title
        title="Applications"
        count={filteredApplications.length}
        extra={
          <div className="flex gap-2">
            <Input
              type="text"
              placeholder="Filter"
              value={filterApplication}
              onChange={(e) => setFilterApplication(e.target.value)}
            />
            <Button className="bg-emerald-400">
              <Filter />
            </Button>
          </div>
        }
      />
      <div className="grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 grid gap-2">
        {filteredApplications.map((application) => (
          <ApplicationCard key={application.id} application={application} />
        ))}
        <Link
          to="/apps/create"
          className="items-center border border-emerald-400 border-dashed p-4 rounded-xl bg-white/50 dark:bg-black/30 dark:hover:bg-black/60 hover:bg-white transition-all cursor-pointer flex gap-2 shadow-inner"
        >
          <Plus />
          <h2 className="text-md font-bold">Create a new application</h2>
        </Link>
      </div>
    </Layouts.Container>
  );
};

export const Route = createFileRoute("/")({
  component: IndexComponent,
  beforeLoad: beforeLoadAuthenticated(),
  loader: async ({ context: { queryClient } }) =>
    Promise.all([
      queryClient.ensureQueryData(api.queryOptions("get", "/users")),
      queryClient.ensureQueryData(api.queryOptions("get", "/applications")),
    ]),
});
