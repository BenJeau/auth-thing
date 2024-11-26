import { useMemo } from "react";
import {
  BookOpenText,
  Code,
  DoorClosed,
  Fingerprint,
  LucideIcon,
} from "lucide-react";
import { Link, useLocation } from "@tanstack/react-router";
import { useAtomValue } from "jotai";

import { AutoAnimate, Layouts, Trans } from "@/components";
import { getRandomBackground } from "@/assets/background";
import { TransId, useTranslation } from "@/i18n";
import { cn } from "@/lib/utils";
import config from "@/lib/config";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { userAtom } from "@/atoms/auth";

const UserAvatar = () => {
  const user = useAtomValue(userAtom);

  if (!user) {
    return null;
  }

  return (
    <AutoAnimate
      slideIn
      className="absolute top-5 right-4 lg:top-10 lg:right-10"
    >
      <DropdownMenu>
        <DropdownMenuTrigger className="flex items-center gap-2">
          <div className="flex flex-col text-right">
            <span className="text-sm font-semibold">{user.name}</span>
            <span className="text-xs">{user.email}</span>
          </div>
          <Avatar className="shadow-md select-none">
            <AvatarFallback>{user.initials}</AvatarFallback>
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
    </AutoAnimate>
  );
};

export const Authentication: React.FC<React.PropsWithChildren> = ({
  children,
}) => {
  const bg = useMemo(() => getRandomBackground(), []);
  const { t } = useTranslation();
  const location = useLocation();

  return (
    <div className="relative grid max-h-[850px] h-full flex-col items-center justify-center lg:max-w-none lg:grid-cols-5 lg:ps-4">
      <AutoAnimate
        slideIn
        className="col-span-3 hidden h-full flex-col gap-4 lg:flex py-8"
      >
        <div className="relative h-full flex-grow overflow-hidden rounded-3xl bg-muted text-white shadow-lg shadow-primary/20 dark:shadow-primary/5">
          <img
            id="lowres-login-img"
            src={bg.small}
            alt={t("login.image.blurry.alt")}
            className="absolute h-full w-full flex-1 object-cover blur-xl"
          />
          <img
            id="login-img"
            src={bg.big}
            alt={t("login.image.alt")}
            className="absolute h-full flex-1 w-full object-cover opacity-0 blur-xl transition duration-1000 ease-out"
            style={{ transitionProperty: "filter" }}
            onLoad={() => {
              setTimeout(() => {
                document
                  .querySelector("#lowres-login-img")
                  ?.classList.add("opacity-0");
                document
                  .querySelector("#login-img")
                  ?.classList.remove("blur-xl");
                document
                  .querySelector("#login-img")
                  ?.classList.remove("opacity-0");
              }, 500);
            }}
          />
          <div className="absolute inset-0 flex flex-col">
            <div className="relative z-20 flex flex-col gap-4 p-10 pb-0">
              <div className="flex items-center gap-4 rounded-xl bg-gradient-to-r from-primary to-accent bg-clip-text pb-1 text-5xl font-extrabold text-transparent">
                <Fingerprint
                  size={48}
                  className="hidden min-w-12 text-primary xl:block"
                  strokeWidth={2.5}
                />
                {config.name}
              </div>
              <Trans id="tagline" />
            </div>
            <div className="relative z-20 mt-auto p-10">
              <p className="text-lg font-medium">
                <Trans id="login.description" />
              </p>
            </div>
            <div className="absolute bottom-0 left-0 right-0 top-0 rounded-xl bg-gradient-to-t from-transparent via-transparent to-black opacity-90" />
            <div className="absolute bottom-0 left-0 right-0 top-0 rounded-xl bg-gradient-to-t from-black via-transparent to-transparent opacity-70 blur-md transition-all duration-500 hover:blur-none" />
          </div>
        </div>
        <div className="flex">
          <SubAction
            title={t("login.subsection.documentation.title")}
            icon={BookOpenText}
            description="login.subsection.documentation.description"
            href="https://docs.auththing.dev"
            className="rounded-e-none"
          />
          <div className="w-[1px] bg-primary dark:bg-primary/90" />
          <SubAction
            title={t("login.subsection.code.title", { name: config.name })}
            icon={Code}
            description="login.subsection.code.description"
            href="https://github.com/auththing/auththing"
            className="rounded-s-none"
          />
        </div>
        <AutoAnimate slideIn className="flex items-center justify-between mt-2">
          <Layouts.Public.Footer />
        </AutoAnimate>
      </AutoAnimate>
      <div className="absolute left-4 top-4 lg:hidden">
        <div className="flex flex-wrap items-center gap-4 rounded-xl bg-gradient-to-r from-primary to-accent bg-clip-text pb-1 text-3xl font-extrabold text-transparent sm:gap-4 sm:text-5xl">
          <Fingerprint
            size={48}
            className="hidden min-w-12 text-primary sm:block"
            strokeWidth={2.5}
          />
          {config.name}
        </div>
      </div>
      <div className="lg:col-span-2 lg:p-8 lg:relative h-full">
        <AutoAnimate
          key={location.pathname}
          slideIn
          className="absolute inset-0 mx-auto flex w-full flex-col justify-center space-y-8 sm:w-[350px]"
        >
          {children}
        </AutoAnimate>
        <UserAvatar />
      </div>
      <div className="absolute bottom-0 left-4 right-4 mt-8 flex flex-wrap items-center justify-between gap-4 lg:hidden">
        <Layouts.Public.Footer />
      </div>
    </div>
  );
};

interface SubActionProps {
  title: React.ReactNode;
  icon: LucideIcon;
  description: TransId;
  href: string;
  className?: string;
}

const SubAction: React.FC<SubActionProps> = ({
  title,
  icon: Icon,
  description,
  href,
  className,
}) => (
  <a
    className={cn(
      "flex-1 flex-col flex gap-2 rounded-3xl bg-primary/50 dark:bg-primary text-black shadow-lg shadow-primary/20 dark:shadow-primary/5 p-5 transition-all hover:shadow-primary/30 active:shadow-primary/10 dark:hover:shadow-primary/10 cursor-pointer group dark:hover:bg-primary/90 hover:bg-primary/80",
      className,
    )}
    href={href}
    rel="noreferrer noopener"
  >
    <div className="flex items-center gap-2 group-hover:underline">
      <Icon />
      <h2 className="font-semibold text-lg">{title}</h2>
    </div>
    <p className="text-sm">
      <Trans id={description} />
    </p>
  </a>
);
