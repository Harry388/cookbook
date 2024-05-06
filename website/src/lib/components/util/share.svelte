<script lang="ts">

    import { page } from '$app/stores';

    export let path: string = '';
    
    $: url = path ? $page.url.origin + path : $page.url.href;

    async function share() {
        if (navigator.share) {
            await navigator.share({
                title: 'Cookbook',
                text: 'Check out this post from cookbook!',
                url
            })
        }
        else {
            const permission = await  navigator.permissions.query({ name: "clipboard-write" as PermissionName })
            if (permission.state == "granted" || permission.state == "prompt") {
                navigator.clipboard.writeText(url);
            }
        }
    }

</script>

<button class="fa-regular fa-share-from-square text-2xl" on:click={share}></button>
