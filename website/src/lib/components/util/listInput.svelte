<script lang="ts">

    export let list: string[] = [''];

    function add() {
        list = [...list, ''];
    }

    function remove(index: number) {
        list = list.filter((_, i) => index != i);
    }

</script>

{#each list as item, i}
    <div class="flex items-center">
        <p class="mr-5">{i + 1}.</p>
        <form on:submit={add}>
            <input autofocus={(i == (list.length - 1)) && !item} type="text" class="input input-bordered" bind:value={list[i]} placeholder="{i + 1}.">
        </form>
        <button class="btn btn-outline w-fit" on:click={() => remove(i)}>Remove</button>
    </div>
{/each}

<button class="btn btn-outline w-fit" on:click={add}>Add</button>