<script lang="ts">

    import Cookbooks from '$lib/components/cookbook/cookbooks.svelte';

    export let data;

</script>

<div class="lg:w-1/3 lg:m-auto flex flex-col">
    {#if data.self}
        <a href="/cookbook/create" class="btn btn-outline"><i class="fa-solid fa-plus"></i>Create Cookbook</a>
    {/if}
    {#if data.cookbooks.length == 0}
        <p class="text-lg text-center">No cookbooks found</p>
    {/if}
    <Cookbooks cookbooks={data.cookbooks} />
</div>
