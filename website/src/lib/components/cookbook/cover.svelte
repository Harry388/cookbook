<script lang="ts">

    import type { Cookbook } from '$lib/app/cookbook';

    export let cookbook: Cookbook;

</script>

<div class="hero pt-80 bg-base-200">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold">{ cookbook.title }</h1>
            <h2 class="text-3xl pt-5">Created By <a href="/user/{cookbook.user_id}" target="_blank" class="link">{ cookbook.user_display_name }</a></h2>
        </div>
    </div>
</div>
