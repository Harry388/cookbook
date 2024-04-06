import { searchCommunities } from '$lib/app/search';

export const load = async ({ parent, fetch }) => {

    const { search } = await parent();

    const communities = search ? searchCommunities(search).json(fetch) : '';

    return {
        communities
    }

}
