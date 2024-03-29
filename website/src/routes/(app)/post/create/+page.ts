import { getUserCommunities } from '$lib/app/community';
import { getUserRecipes } from '$lib/app/recipe';

export const load = async ({ parent, fetch, url }) => {

    const { id } = await parent();
    const communityRaw = Number(String(url.searchParams.get('c')));
    const community = isNaN(communityRaw) ? null : communityRaw;

    const [
        communities,
        userRecipes
    ] = await Promise.all([
        getUserCommunities(id).json(fetch),
        getUserRecipes(id).json(fetch)
    ]);

    //@ts-ignore
    const communityChecked = communities.map(c => c.id).includes(community) ? community : null;

    return {
        communities,
        userRecipes,
        community: communityChecked
    }
}
