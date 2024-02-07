import { getUserCommunities } from '$lib/app/community';

export const load = async ({ parent, fetch }) => {

    const { id } = await parent();

    const communities = await getUserCommunities(id, fetch);

    return {
        communities
    }
}
