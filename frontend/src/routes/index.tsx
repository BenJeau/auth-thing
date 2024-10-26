import { createFileRoute } from "@tanstack/react-router";
import { Filter } from "lucide-react";
import { ColumnDef } from "@tanstack/react-table";

import { api, components } from "@/api";
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
    <div className="rounded-xl border  shadow-inner bg-white">
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

const userTableColumnDef: ColumnDef<components["schemas"]["User"]>[] = [
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

const IndexComponent: React.FC = () => {
  const users = api.useSuspenseQuery("get", "/users");

  return (
    <div className="flex gap-4 flex-col bg-emerald-100 dark:bg-emerald-950/30 rounded-t-2xl pt-6 shadow-inner border border-b-0 border-emerald-400 dark:border-emerald-900 flex-1">
      <div className="flex justify-between gap-2 px-6 flex-wrap">
        <h1 className="text-xl font-bold flex gap-2 items-baseline">
          Applications
          <EntryCount count={3} />
        </h1>
        <div className="flex gap-2">
          <Input type="text" placeholder="Filter" />
          <Button>
            <Filter />
          </Button>
        </div>
      </div>
      <div className="flex gap-4 flex-col bg-cyan-100 -mx-[1px] dark:bg-cyan-950/30 rounded-t-2xl p-6 shadow-inner border border-b-0 border-cyan-400 dark:border-cyan-900 flex-1">
        <div className="flex justify-between gap-2 flex-wrap">
          <h1 className="text-xl font-bold flex gap-2 items-baseline">
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
    queryClient.ensureQueryData(api.queryOptions("get", "/users")),
});
