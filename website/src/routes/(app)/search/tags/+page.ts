import { searchTags } from '$lib/app/search';

export const load = async ({ parent, fetch }) => {

    const { search } = await parent();

    const tags = await searchTags(search).json(fetch);

    return {
        tags
    }

}
