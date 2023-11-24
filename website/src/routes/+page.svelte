<script>

    let text = '';

    async function login() {
        await fetch("http://localhost:8000/api/auth/login", {
            method: "post",
            body: JSON.stringify({
                email: "harry@example.com",
                password: "password"
            }),
            headers: {
                'Accept': '*',
                'Content-Type': 'application/json'
            },
            credentials: 'include'
        });
    }

    async function logout() {
        await fetch("http://localhost:8000/api/auth/logout", { method: 'post', credentials: 'include' })
    }

    async function test() {
        const response = await fetch("http://localhost:8000/api/auth/test", { credentials: 'include'});
        text = await response.json();
    }

</script>

<button on:click={login}>Log In</button>
<button on:click={logout}>Log Out</button>
<button on:click={test}>Test</button>
{ text }