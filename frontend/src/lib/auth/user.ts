// lib/stores/user.svelte.ts
export enum Roles {
    USER = "user",
    ADMIN = "admin",
}

export type AuthUser = {
    id: string;
    username: string;
    avatar?: string | null;
    role: Roles;
};
