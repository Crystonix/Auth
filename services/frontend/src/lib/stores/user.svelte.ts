import type { AuthUser, Roles } from "$lib/auth/user";
import colorthief from "colorthief";

class User {
    id: string = $state('');
    username: string = $state('');
    avatar: string | null = $state(null);
    role: Roles | null = $state(null);
    authenticated: boolean = $state(false);
    accentColor: string = $state('blue');
    contrastColor: string = $state('white');

    setUser(user: AuthUser) {
				console.log('User.setUser called with:', user);
        this.id = user.id;
        this.username = user.username;
        this.avatar = user.avatar ?? null;
        this.role = user.role;
        this.authenticated = true;
        this.fetchAccentColor();
        this.computeContrastColor(this.accentColor);
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

    async fetchAccentColor() {
        if (this.avatar) {
            try {
                const img = new Image();
                img.crossOrigin = "Anonymous";
                img.src = this.avatar;
                await img.decode();

                const colorThief = new colorthief();
                const dominantColor = colorThief.getColor(img);
                this.accentColor = `rgb(${dominantColor[0]}, ${dominantColor[1]}, ${dominantColor[2]})`;

                this.contrastColor = this.computeContrastColor(this.accentColor);
            } catch (error) {
                console.warn("Error fetching accent color:", error);
                this.accentColor = 'blue';
            }
        } else {
            this.accentColor = 'blue';
            this.contrastColor = 'white';
        }
    }

    private computeContrastColor(rgb: string): string {
        const result = rgb.match(/\d+/g);
        if (result && result.length === 3) {
            const r = parseInt(result[0], 10);
            const g = parseInt(result[1], 10);
            const b = parseInt(result[2], 10);
            const brightness = (r * 299 + g * 587 + b * 114) / 1000;
            return brightness > 125 ? 'black' : 'white';
        }
        return 'white';
    }

}

export const user = new User();