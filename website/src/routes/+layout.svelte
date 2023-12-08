<script>
    import "../app.css";
    import { page } from '$app/stores';
</script>

<svelte:head>
    <title>{ $page.data.title ? `${$page.data.title} -` : '' } CookBook</title>
</svelte:head>

<slot />