import { getUserCommunities } from '$lib/app/community';

export const load = async ({ fetch, parent, depends }) => {

    const { id } = await parent();

    const communities = await getUserCommunities(id).json(fetch);

    depends('app:community');

    return {
        communities
    }

}
