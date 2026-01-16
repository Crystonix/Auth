// src/routes/+layout.server.ts
import type { LayoutServerLoad } from './$types';
import type { AuthUser } from '$lib/auth/user';

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
  const cookieHeader = cookies
    .getAll()
    .map(c => `${c.name}=${c.value}`)
    .join('; ');

  const res = await fetch('http://auth-service:4000/me', {
    headers: { cookie: cookieHeader }
  });

  console.log("Res: ", res, "Cookie Header: ", cookieHeader);

  if (!res.ok) {
    return { user: null };
  }

  const user: AuthUser = await res.json();

  return { user };
};
