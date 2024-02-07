import { getCommunityMembers } from '$lib/app/communityMember';
import { getCommunity } from '$lib/app/community';

export const load = async ({ params, fetch }) => {
    
    const [community, members] = await Promise.all([getCommunity(params.id, fetch), getCommunityMembers(params.id, fetch)]);

    return {
        community,
        members
    }
}
