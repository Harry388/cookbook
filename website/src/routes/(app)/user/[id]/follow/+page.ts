import { getUserFollow, getRequests } from '$lib/app/follow';
import { getUser } from '$lib/app/user';
import { redirect } from '@sveltejs/kit';
import type { Follow } from '$lib/app/follow';

export const load = async ({ params, fetch, parent, depends }) => {
    
    const { id } = await parent();

    const user = await getUser(params.id).json(fetch);

    const isSelf = id == Number(params.id);

    if (!isSelf && !user.public && !user.is_following) {
        throw redirect(301, `/user/${user.id}`);
    }

    const { followers, following } = await getUserFollow(params.id).json(fetch);

    let requests: Follow[] = [];
    
    if (id == params.id) {
        requests = await getRequests().json(fetch);
    }

    depends('app:userFollow');

    return {
        followers,
        following,
        requests,
        self: isSelf,
        userId: Number(params.id),
        title: user.display_name
    }
}
