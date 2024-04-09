<script lang="ts">

    import ListInput from '$lib/components/util/listInput.svelte';
    import TagInput from '$lib/components/tag/tagInput.svelte';
    import Input from '$lib/components/util/input.svelte';
    import type { Tag } from '$lib/app/tag';

    export let title = '';
    export let description = '';
    export let ingredients: string[];
    export let method: string[];
    export let tags: Tag[];

</script>

<div class="form-control">
    <Input bind:value={title} title="Title" required />
    <Input bind:value={description} title="Description" long />
    <ListInput bind:list={ingredients} title="Ingredients" placeholder="Next Ingredient" />
    <ListInput bind:list={method} title="Method" placeholder="Next Step" long />
    <TagInput bind:tags={tags} />
</div>
