<script>

    import { login, logout, test } from '$lib/auth/auth';

    let email = '';
    let password = '';

    let id = '';

    async function setId() {
        const response = await test();
        const resId = await response.json();
        id = resId;
    }

</script>

<input type="text" bind:value={email}>
<input type="text" bind:value={password}>
<button on:click={() => login(email, password)}>Log In</button>
<button on:click={logout}>Log Out</button>
<button on:click={setId}>Test</button>
{ id }
<button on:click={() => id = ''}>Clear</button>