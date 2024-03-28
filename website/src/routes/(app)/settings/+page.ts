import { getUser } from '$lib/app/user'

export const load = async ({ parent, fetch, depends }) => {

    const { id } = await parent();

    const user = await getUser(id).json(fetch);

    depends('app:settings');

    return {
        user,
        title: 'Settings',
    }
}
