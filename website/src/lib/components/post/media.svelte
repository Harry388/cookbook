<script lang="ts">

    import Image from '$lib/components/util/image.svelte';

    export let id: number;

</script>

<Image src={`post/media/${id}`} alt="Post Image" />