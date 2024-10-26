import { Link, Outlet } from "@tanstack/react-router";

import { useUpdateTheme } from "@/atoms/theme";

const Layout: React.FC = () => {
  useUpdateTheme();

  return (
    <div className="container mx-auto p-4 flex flex-col gap-4">
      <div className="p-2 flex gap-2">
        <Link to="/" className="[&.active]:font-bold">
          Home
        </Link>{" "}
        <Link to="/about" className="[&.active]:font-bold">
          About
        </Link>
      </div>
      <Outlet />
    </div>
  );
};

export default Layout;
