import { getUserFollow, getRequests } from '$lib/app/follow';
import { getUser } from '$lib/app/user';
import type { Follow } from '$lib/app/follow';

export const load = async ({ params, fetch, parent, depends }) => {
    
    const { id } = await parent();

    const user = await getUser(params.id).json(fetch);

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
        self: id == Number(params.id),
        userId: Number(params.id),
        title: user.display_name
    }
}
