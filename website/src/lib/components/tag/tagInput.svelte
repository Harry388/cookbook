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
        if (tags.length <= 9) {
            if (newTag == '') return;
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

<form on:submit|preventDefault={add} class="form-control">
    <label class="label" for="#input">
        <span class="label-text">
            Tags
            {#if tags.length >= 7}
                <span class="text-error">{ tags.length } / 10</span>
            {/if}
        </span>
    </label>
    <div class="flex">
        <input id="input" type="text" min="1" bind:value={newTag} placeholder="Tag" class="input input-bordered mr-2 w-full" />
        <button on:click={add} class="btn btn-outline"><i class="fa-solid fa-plus"></i></button>
    </div>
    {#if tags.length}
        <div class="flex flex-wrap gap-2 my-5">
            {#each tags as tag (tag.id)}
                <TagComponent {tag} on:remove={remove} edit />
            {/each}
        </div>
    {/if}
</form>
