import { getCommunity } from '$lib/app/community';

export const load = async ({ params, fetch }) => {

    const community = await getCommunity(params.id, fetch);

    return {
        community,
        title: community.title
    }
}

