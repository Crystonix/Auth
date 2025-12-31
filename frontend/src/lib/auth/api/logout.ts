export async function logout() {
	try {
		await fetch('/api/auth/logout', {
			method: 'POST',
			credentials: 'include'
		});
	} catch(e) {
		console.log(e)
	}
}
