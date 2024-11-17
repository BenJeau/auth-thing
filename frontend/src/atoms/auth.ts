import { atomWithLocalStorage } from "@/atoms";

export interface User {
  id: number;
  email: string;
  name?: string;
  givenName?: string;
  familyName?: string;
  roles: string[];
  token: string;
  initials: string;
  emailVerified: boolean;
}

export const userAtom = atomWithLocalStorage<User | undefined>(
  "user",
  undefined
);
