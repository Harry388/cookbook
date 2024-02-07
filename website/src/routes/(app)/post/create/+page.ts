import { getUserCommunities } from '$lib/app/community';

export const load = async ({ parent, fetch, url }) => {

    const { id } = await parent();
    const communityRaw = Number(String(url.searchParams.get('c')));
    const community = isNaN(communityRaw) ? null : communityRaw;

    const communities = await getUserCommunities(id, fetch);

    //@ts-ignore
    const communityChecked = communities.map(c => c.id).includes(community) ? community : null;

    return {
        communities,
        community: communityChecked
    }
}
