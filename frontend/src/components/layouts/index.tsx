import { Link, Outlet } from "@tanstack/react-router";

import { useUpdateTheme } from "@/atoms/theme";

const Layout: React.FC = () => {
  useUpdateTheme();

  return (
    <div className="container mx-auto flex flex-col gap-6 mt-6">
      <div className="flex gap-2">
        <Link to="/" className="[&.active]:font-bold">
          Home
        </Link>{" "}
        <Link to="/about" className="[&.active]:font-bold">
          About
        </Link>
      </div>
      <main className="flex-1 flex flex-col">
        <Outlet />
      </main>
    </div>
  );
};

export default Layout;
