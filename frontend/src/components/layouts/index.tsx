import { Link, Outlet } from "@tanstack/react-router";
import { DoorClosed, Fingerprint } from "lucide-react";
import { useAtomValue } from "jotai";

import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { useUpdateTheme } from "@/atoms/theme";
import { userAtom } from "@/atoms/auth";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

import Container from "./container";

const Layout: React.FC = () => {
  const user = useAtomValue(userAtom);

  useUpdateTheme();

  return (
    <div className="container mx-auto flex flex-col gap-6 mt-6 px-4">
      <div className="flex gap-4 items-center justify-between">
        <Link
          to="/"
          className="flex gap-4 font-bold text-xl items-center hover:underline"
        >
          <Fingerprint size={32} />
          Auth Thing
        </Link>

        {user && (
          <DropdownMenu>
            <DropdownMenuTrigger>
              <Avatar className="shadow-md select-none">
                <AvatarFallback>{user?.initials}</AvatarFallback>
              </Avatar>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuLabel>Manage account</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem
                asChild
                className="focus:bg-destructive focus:text-destructive-foreground text-destructive"
              >
                <Link to="/auth/logout" className="flex gap-2 items-center">
                  <DoorClosed size={16} />
                  <span>Logout</span>
                </Link>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>

      <main className="flex-1 flex flex-col">
        <Outlet />
      </main>
    </div>
  );
};

export { Container };
export default Layout;
