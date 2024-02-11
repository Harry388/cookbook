<script>

    import { page } from '$app/stores';

</script>

<div class="hero min-h-screen bg-base-200">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="text-5xl font-bold">Error: { $page.status }</h1>
        <p class="py-6">{ $page.error?.message }</p>
        <a href="/" class="btn btn-primary">Go Home</a>
        <button class="btn btn-primary" on:click={() => history.back()}>Go Back</button>
      </div>
    </div>
</div>
