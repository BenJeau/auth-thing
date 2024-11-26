import { Home, Languages } from "lucide-react";
import { Link, Outlet } from "@tanstack/react-router";
import { useAtom } from "jotai";

import { Button } from "@/components/ui/button";
import { ThemeIcon, themeAtom, Theme } from "@/atoms/theme";
import { useTranslation } from "@/i18n";
import { ApplicationVersion, Trans } from "@/components";
import config from "@/lib/config";

export const Layout: React.FC = () => (
  <div className="container relative mx-auto flex min-h-screen flex-col justify-center p-4 dark:text-white">
    <Outlet />
  </div>
);

export const Footer: React.FC<{ showHome?: boolean }> = ({ showHome }) => {
  const [theme, setTheme] = useAtom(themeAtom);
  const { toggle, otherLang } = useTranslation();

  return (
    <>
      <div className="text-sm text-black/50 dark:text-white/50">
        &copy; {new Date().getFullYear()} {config.domain} -{" "}
        <ApplicationVersion />
      </div>
      <div className="flex flex-wrap gap-10">
        {showHome && (
          <Link to="/">
            <Button variant="link" className="gap-2 lowercase">
              <Home size={16} />
              <Trans id="home" />
            </Button>
          </Link>
        )}
        <div className="flex gap-4">
          {Object.entries(ThemeIcon).map(([key, Icon]) => (
            <Button
              key={key}
              data-selected={theme === key}
              disabled={theme === key}
              variant="link"
              className="px-0 text-black transition-all disabled:opacity-100 data-[selected=true]:text-primary data-[selected=false]:hover:opacity-50 dark:text-white dark:data-[selected=true]:text-primary"
              onClick={() => {
                setTheme(key as Theme);
              }}
            >
              <Icon size={16} />
            </Button>
          ))}
        </div>
        <Button
          variant="link"
          className="gap-2 p-0 text-sm text-black dark:text-white"
          onClick={toggle}
        >
          <Languages size={16} />
          <span className="w-5">{otherLang.lang.short}</span>
        </Button>
      </div>
    </>
  );
};
