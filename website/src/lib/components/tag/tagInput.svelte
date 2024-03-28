<script lang="ts">

    import { createEventDispatcher } from 'svelte';
    import TagComponent from '$lib/components/tag/tag.svelte';
    import type { Tag } from '$lib/app/tag';

    const dispatch = createEventDispatcher();

    export let tags: Tag[] = [];
    export let edit = false;

    let newTag = '';
    let count = 0;

    function add() {
        if (tags.map(t => t.tag).includes(newTag)) {
            newTag = '';
            return;
        }
        if (!edit) {
            tags = [...tags, {
                tag: newTag,
                id: count,
                is_following: 0
            }];
        }
        else {
            dispatch('add', newTag);
        }
        newTag = '';
        count++;
    }

    function remove(event: CustomEvent<Tag>) {
        if (!edit) {
            tags = tags.filter(t => t.id != event.detail.id);
        }
        else {
            dispatch('remove', event.detail);
        }
    }

</script>

<div class="flex">
    <form on:submit={add}>
        <label class="label" for="#input">
            <span class="label-text">Tags</span>
        </label>
        <input id="input" type="text" min="1" bind:value={newTag} placeholder="Tag" class="input input-bordered" />
        <input type="submit" class="btn btn-primary w-fit mt-5" value="Add" />
    </form>
    {#each tags as tag}
        <TagComponent {tag} on:remove={remove} edit />
    {/each}
</div>
