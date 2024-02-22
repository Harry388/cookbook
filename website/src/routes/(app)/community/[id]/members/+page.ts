import { getCommunityMembers, getCommunityRequests } from '$lib/app/communityMember';
import { redirect } from '@sveltejs/kit';
import type { CommunityMember } from '$lib/app/communityMember';

export const load = async ({ fetch, params, depends, parent }) => {

    const { community } = await parent();

    if (!community.is_member) {
        throw redirect(301, `/community/${community.id}`);
    }

    const members = await getCommunityMembers(params.id).json(fetch);

    let requests: CommunityMember[] = [];

    if (community.is_admin) {
        requests = await getCommunityRequests(params.id).json(fetch);
    }

    depends('app:communityMembers');
    
    return {
        members,
        requests
    }

}
