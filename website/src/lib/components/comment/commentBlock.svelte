<script lang="ts">

    import Comments from '$lib/components/comment/comments.svelte';
    import AddComment from '$lib/components/comment/addComment.svelte';
    import type { Comment } from '$lib/app/comment';

    export let comments: Comment[];
    export let type: 'POST' | 'RECIPE';
    export let id: number;

    $: depends = `app:${type.toLowerCase()}`;

</script>

<Comments {comments} {type} />
<AddComment {type} {id} {depends} />
