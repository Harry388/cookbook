<script lang="ts">

    import type { Section } from '$lib/app/cookbook';

    export let section: Section;

</script>

<div class="flex flex-col items-center gap-5">
    <h1 class="font-bold text-3xl">{ section.position + 1 }. { section.title }</h1>
</div>
