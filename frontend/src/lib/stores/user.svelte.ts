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

class User {
    id: string = $state('');
    username: string = $state('');
    avatar: string | null = $state(null);
    role: Roles | null = $state(null);
    authenticated: boolean = $state(false);

    setUser(user: AuthUser) {
        this.id = user.id;
        this.username = user.username;
        this.avatar = user.avatar ?? null;
        this.role = user.role;
        this.authenticated = true;
    }

    setAuthenticated(role: Roles) {
        this.role = role;
        this.authenticated = true;
    }

    reset() {
        this.id = '';
        this.username = '';
        this.avatar = null;
        this.role = null;
        this.authenticated = false;
    }
}

export const user = new User();
