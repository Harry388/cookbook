<script lang="ts">

    import type { Cookbook } from '$lib/app/cookbook';

    export let cookbook: Cookbook;

</script>

<a target="_blank" href="/cookbook/{cookbook.id}">{ cookbook.title }</a>
