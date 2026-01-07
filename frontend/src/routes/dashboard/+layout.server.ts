// src/routes/dashboard/+layout.server.ts
import { redirect, type Load } from '@sveltejs/kit';

export const load: Load = async ({ parent }) => {
  const { user } = await parent(); // get user from root layout

  if (!user) {
    throw redirect(302, '/'); // not logged in
  }

  return { user }; // pass to dashboard children
};
