import { searchCommunities } from '$lib/app/search';

export const load = async ({ parent, fetch }) => {

    const { search } = await parent();

    const communities = searchCommunities(search).json(fetch);

    return {
        communities
    }

}
