import { getCommunityMembers } from '$lib/app/communityMember';

export const load = async ({ fetch, params, depends }) => {

    const members = await getCommunityMembers(params.id).json(fetch);

    depends('app:communityMembers');
    
    return {
        members
    }

}
