import type { AuthUser, Roles } from "$lib/auth/user";

class User {
    id: string = $state('');
    username: string = $state('');
    avatar: string | null = $state(null);
    role: Roles | null = $state(null);
    authenticated: boolean = $state(false);

    setUser(user: AuthUser) {
				console.log('User.setUser called with:', user);
        this.id = user.id;
        this.username = user.username;
        this.avatar = user.avatar ?? null;
        this.role = user.role;
        this.authenticated = true;
    }

    setAuthenticated(role: Roles) {
				console.log('User.setAuthenticated called with role:', role);
        this.role = role;
        this.authenticated = true;
    }

    reset() {
				console.log('User.reset called');
        this.id = '';
        this.username = '';
        this.avatar = null;
        this.role = null;
        this.authenticated = false;
    }
}

export const user = new User();