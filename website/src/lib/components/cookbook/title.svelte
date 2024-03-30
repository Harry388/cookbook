<script lang="ts">

    import type { Cookbook } from '$lib/app/cookbook';

    export let cookbook: Cookbook;

</script>

<div class="hero pt-80 bg-base-200">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold">{ cookbook.title }</h1>
            <p class="text-2xl py-6">{ cookbook.description }</p>
        </div>
    </div>
</div>
