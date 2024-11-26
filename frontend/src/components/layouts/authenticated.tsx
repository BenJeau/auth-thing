import { Link, Outlet } from "@tanstack/react-router";
import { DoorClosed, Fingerprint } from "lucide-react";

import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { User } from "@/atoms/auth";

type Props = {
  user: User;
};

export const Layout: React.FC<Props> = ({ user }) => {
  return (
    <div className="container mx-auto mt-6 flex flex-col gap-6 px-4">
      <div className="flex items-center justify-between gap-4">
        <Link
          to="/"
          className="flex items-center gap-4 text-xl font-bold hover:underline"
        >
          <Fingerprint size={32} />
          Auth Thing
        </Link>

        <DropdownMenu>
          <DropdownMenuTrigger className="flex items-center gap-2">
            <div className="flex flex-col text-right">
              <span className="text-sm font-semibold">{user.name}</span>
              <span className="text-xs">{user.email}</span>
            </div>
            <Avatar className="select-none shadow-md">
              <AvatarFallback>{user.initials}</AvatarFallback>
            </Avatar>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuLabel>Manage account</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              asChild
              className="text-destructive focus:bg-destructive focus:text-destructive-foreground"
            >
              <Link to="/auth/logout" className="flex items-center gap-2">
                <DoorClosed size={16} />
                <span>Logout</span>
              </Link>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <main className="flex flex-1 flex-col">
        <Outlet />
      </main>
    </div>
  );
};
