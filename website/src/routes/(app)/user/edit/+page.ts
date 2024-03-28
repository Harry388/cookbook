import { getUser } from '$lib/app/user';

export const load = async ({ parent, fetch, depends }) => {

    const { id } = await parent();

    const user = await getUser(id).json(fetch);

    depends('app:user');

    return {
        user,
        title: user.display_name,
        self: id == user.id
    }
}
