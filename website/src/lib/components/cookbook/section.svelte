<script lang="ts">

    import type { Section } from '$lib/app/cookbook';

    export let section: Section;

</script>

<div class="hero lg:pt-80 print:pt-80 bg-base-200">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold">{ section.position + 1}. { section.title }</h1>
            {#if section.description}
                <p class="text-2xl py-6">{ section.description }</p>
            {/if}
        </div>
    </div>
</div>
