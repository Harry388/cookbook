import { getUserFollow } from '$lib/app/follow';
import { getUser } from '$lib/app/user';

export const load = async ({ params, fetch, parent, depends }) => {
    
    const { id } = await parent();

    const user = await getUser(params.id).json(fetch);

    const { followers, following } = await getUserFollow(params.id).json(fetch);

    depends('app:userFollow');

    return {
        followers,
        following,
        self: id == Number(params.id),
        userId: Number(params.id),
        title: user.display_name
    }
}
