import { Link, Outlet } from "@tanstack/react-router";
import { Fingerprint } from "lucide-react";

import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { useUpdateTheme } from "@/atoms/theme";

const Layout: React.FC = () => {
  useUpdateTheme();

  return (
    <div className="container mx-auto flex flex-col gap-6 mt-6">
      <div className="flex gap-4 items-center justify-between">
        <Link
          to="/"
          className="flex gap-4 font-bold text-xl items-center hover:underline"
        >
          <Fingerprint strokeWidth={2.5} size={28} />
          Auth Thing
        </Link>

        <Avatar>
          <AvatarImage src="https://github.com/shadcn.png" />
          <AvatarFallback>CN</AvatarFallback>
        </Avatar>
      </div>

      <main className="flex-1 flex flex-col">
        <Outlet />
      </main>
    </div>
  );
};

export default Layout;
