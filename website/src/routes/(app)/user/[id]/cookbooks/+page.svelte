<script lang="ts">

    import Cookbooks from '$lib/components/cookbook/cookbooks.svelte';

    export let data;

</script>

<Cookbooks cookbooks={data.cookbooks} />
