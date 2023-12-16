import { getUserFollow } from '$lib/app/follow';
import { getUser } from '$lib/app/user';

export const load = async ({ params, fetch, parent }) => {
    
    const { id } = await parent();

    const user = await getUser(params.id, fetch);

    const { followers, following } = await getUserFollow(params.id, fetch);

    return {
        followers,
        following,
        self: id == Number(params.id),
        userId: Number(params.id),
        title: user.display_name
    }
}