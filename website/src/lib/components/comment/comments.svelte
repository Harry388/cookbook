<script lang="ts">

    import CommentComponent from '$lib/components/comment/comment.svelte';
    import type { Comment } from '$lib/app/comment';

    export let comments: Comment[]
    export let type: 'POST' | 'RECIPE';

</script>

{#each comments as comment (comment.id)}
   <CommentComponent {comment} {type} /> 
{/each}
