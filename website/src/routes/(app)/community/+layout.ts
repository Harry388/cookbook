import { getUserCommunities } from '$lib/app/community';

export const load = async ({ fetch, parent }) => {

    const { id } = await parent();

    const communities = await getUserCommunities(id, fetch);

    return {
        communities,
        title: 'Communities'
    }

}
