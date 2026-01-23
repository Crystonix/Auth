// src/routes/+layout.server.ts
import type { LayoutServerLoad } from './$types';
import type { AuthUser } from '$lib/auth/user';

const AUTH_URL =
  process.env.AUTH_SERVICE_URL ||
  'http://localhost:4000';

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
  const cookieHeader = cookies
    .getAll()
    .map(c => `${c.name}=${c.value}`)
    .join('; ');

  const res = await fetch('${AUTH_URL}/me', {
    headers: { cookie: cookieHeader }
  });

  console.log("Res: ", res, "Cookie Header: ", cookieHeader);

  if (!res.ok) {
    return { user: null };
  }

  const user: AuthUser = await res.json();

  return { user };
};
