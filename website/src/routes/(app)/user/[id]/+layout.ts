import { getUser } from '$lib/app/user';
import { redirect } from '@sveltejs/kit';

export const load = async ({ params, fetch, route, parent, depends }) => {

    const { id } = await parent();

    if (route.id == '/(app)/user/[id]') {
        throw redirect(301, `/user/${params.id}/posts`);
    }
    
    const user = await getUser(params.id).json(fetch);

    depends('app:user');

    return {
        user,
        title: user.display_name,
        path: route.id.split('/').slice(-1)[0], // for tabs
        self: id == user.id
    }
}
