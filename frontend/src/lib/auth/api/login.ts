export async function login() {
	try {
		await fetch('/api/auth/discord/login', {
			method: 'POST',
			credentials: 'include'
		});
	} catch(e) {
		console.log(e)
	}
}