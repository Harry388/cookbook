<script lang="ts">

    import { page } from '$app/stores';

    function share() {
        if (navigator.share) {
            navigator.share({
                title: 'Cookbook',
                text: 'Check out this post from cookbook!',
                url: $page.url.href,
            })
        }
    }

</script>

<button class="btn btn-warning" on:click={share}>Share</button>
