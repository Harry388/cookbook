<script lang="ts">

    import Cookbooks from '$lib/components/cookbook/cookbooks.svelte';

    export let data;

</script>

<div class="lg:w-1/3 lg:m-auto flex flex-col">
    <Cookbooks cookbooks={data.cookbooks} />
</div>
