<script lang="ts">

    import Entries from '$lib/components/entries/entries.svelte';

    export let data;

</script>

<div class="lg:w-1/3 m-auto">

    <Entries entries={data.entries} />

</div>
