<script lang="ts">


    export let data;

</script>

<div class="lg:hidden">
    {#if data.self}
        <a href="/user/{data.user.id}/albums/create" class="mb-5 btn btn-outline w-full"><i class="fa-solid fa-plus"></i>Create Album</a>
    {/if}
    {#each data.albums as album}
        <a href="/user/{data.user.id}/albums/{album.id}" class="card bg-base-100 shadow-xl mb-5">
            <div class="card-body">
                <h2 class="card-title">{ album.title }</h2>
            </div>
        </a>
    {/each}
</div>
