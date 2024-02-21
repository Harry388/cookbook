import { searchUsers } from '$lib/app/search';

export const load = async ({ parent, fetch }) => {

    const { search } = await parent();

    const users = await searchUsers(search).json(fetch);

    return {
        users
    }

}
