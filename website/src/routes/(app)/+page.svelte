<script lang="ts">

    import Entries from '$lib/components/entries/entries.svelte';

    export let data;

</script>

<div class="lg:w-1/3 m-auto">

    {#if data.entries.length == 0}
        <p class="text-lg mt-5">
            Your feed is empty!<br>
            Follow users, communities and tags to see posts and recipes show up here!<br>
            <a href="/search" class="link">Explore</a> new content.
        </p>
    {/if}

    <Entries entries={data.entries} />

</div>
