<script lang="ts">

    import Entries from '$lib/components/entries/entries.svelte';

    export let data;

    console.log(data)

</script>

<div class="w-11/12 lg:w-1/3 m-auto">

    <Entries entries={data.entries} />

</div>
