import type { AuthUser, Roles } from "$lib/auth/user";
import ColorThief from 'colorthief';

class User {
    id: string = $state('');
    username: string = $state('');
    avatar: string | null = $state(null);
    role: Roles | null = $state(null);
    authenticated: boolean = $state(false);
    accentColor: string | null = $state(null);

    setUser(user: AuthUser) {
				console.log('User.setUser called with:', user);
        this.id = user.id;
        this.username = user.username;
        this.avatar = user.avatar ?? null;
        this.role = user.role;
        this.authenticated = true;
        this.accentColor = null; //update when img loaded
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
        this.accentColor = null;
    }

    async getAccentColor(img: HTMLImageElement) {
        try{
            const thief = new ColorThief();
            const [r, g, b] = thief.getColor(img);
            this.accentColor = `rgb(${r}, ${g}, ${b})`;
        }
        catch (err){
            console.error('failed to extract accent color', err);
            this.accentColor = null;
        }
    }
    
}

export const user = new User();