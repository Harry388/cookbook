<script lang="ts">

    import { createPostComment } from '$lib/app/post';
    import { createRecipeComment } from '$lib/app/recipe';
    import { invalidate } from '$app/navigation';

    export let type: 'POST' | 'RECIPE';
    export let id: number;
    export let depends: string;

    let content: string;

    async function save() {
        let response: Response;
        if (type == 'POST') {
            response = await createPostComment(id, content, null).run();
        }
        else {
            response = await createRecipeComment(id, content, null).run();
        }
        if (response.ok) {
            content = '';
            invalidate(depends); 
        }
    }

</script>

<form on:submit={save} class="flex items-center mt-5">
    <input type="text" min="1" bind:value={content} placeholder="New Comment" class="input input-bordered" />
    <button class="fa-regular fa-paper-plane w-fit text-2xl btn btn-ghost"><input type="submit" value="" /></button>
</form>
