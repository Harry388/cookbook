<script lang="ts">

    import { page } from '$app/stores';

    async function share() {
        if (navigator.share) {
            await navigator.share({
                title: 'Cookbook',
                text: 'Check out this post from cookbook!',
                url: $page.url.href,
            })
        }
        else {
            const permission = await  navigator.permissions.query({ name: "clipboard-write" as PermissionName })
            if (permission.state == "granted" || permission.state == "prompt") {
                navigator.clipboard.writeText($page.url.href);
                alert('Copied to clipboard!');
            }
        }
    }

</script>

<button class="btn btn-warning" on:click={share}>Share</button>
