<script lang="ts">

    import TagComponent from '$lib/components/tag/tag.svelte';
    import type { Tag } from '$lib/app/tag';

    export let tags: string[] = [];

    let newTag = '';
    let internalTags: Tag[] = [];
    let count = 0;

    $: tags = internalTags.map(t => t.tag);

    function add() {
        if (tags.includes(newTag)) {
            newTag = '';
            return;
        }
        internalTags = [...internalTags, {
            tag: newTag,
            id: count
        }];
        newTag = '';
        count++;
    }

    function remove(event: CustomEvent<Tag>) {
        internalTags = internalTags.filter(t => t.id != event.detail.id);
    }

</script>

<div class="flex">
    <form on:submit={add}>
        <input type="text" min="1" bind:value={newTag} placeholder="Tag" class="input input-bordered" />
        <input type="submit" class="btn btn-primary w-fit mt-5" value="Add" />
    </form>
    {#each internalTags as tag}
        <TagComponent {tag} on:remove={remove} edit />
    {/each}
</div>
