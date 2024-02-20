<script lang="ts">

    import Entries from '$lib/components/entries/entries.svelte';
    import { followTag, unfollowTag } from '$lib/app/tag';
    import { invalidate } from '$app/navigation';

    export let data;

    async function toggleFollow() {
        if (data.tag.is_following) {
            await unfollowTag(data.tag.id).run();
        }
        else {
            await followTag(data.tag.id).run();
        }
        invalidate('app:tag');
    }

</script>

<button class="btn btn-outline" on:click={toggleFollow}>{ data.tag.is_following ? 'Unfollow' : 'Follow' } Tag</button>

<div class="w-11/12 lg:w-1/3 m-auto">
    <Entries entries={data.entries} />
</div>
