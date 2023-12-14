import { getUser } from '$lib/app/user';

export const load = async ({ parent, fetch }) => {

    const { id } = await parent();

    const user = await getUser(id, fetch);
    
    return {
        user,
        title: 'Settings'
    }
}