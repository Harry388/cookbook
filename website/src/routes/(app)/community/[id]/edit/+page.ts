import { getCommunity } from '$lib/app/community';
import { redirect } from '@sveltejs/kit';

export const load = async ({ params, fetch }) => {

    const community = await getCommunity(params.id, fetch);

    if (!community.is_admin) {
        throw redirect(301, `/community/${community.id}`);
    }

    return {
        community,
        title: community.title
    }
}

