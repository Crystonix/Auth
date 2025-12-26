export enum Roles {
	USER = "user",
	ADMIN = "admin",
}

class User{
	role: Roles | null = $state(null);

	constructor() {

	}
}

export const user = new User();