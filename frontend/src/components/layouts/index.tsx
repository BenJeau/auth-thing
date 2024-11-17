import { useAtomValue } from "jotai";

import { useUpdateTheme } from "@/atoms/theme";
import { userAtom } from "@/atoms/auth";

import { Authentication } from "./authentication";
import Container from "./container";
import * as Public from "./public";
import * as Authenticated from "./authenticated";

const Layout = () => {
  const user = useAtomValue(userAtom);
  useUpdateTheme();

  if (user && user.emailVerified) {
    return <Authenticated.Layout user={user} />;
  } else {
    return <Public.Layout />;
  }
};

export default Layout;

export { Public, Authenticated, Authentication, Container };
