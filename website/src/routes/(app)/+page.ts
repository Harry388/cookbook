import { getUserFeed } from '$lib/app/user';

export const load = async ({ fetch }) => {

    const entries = await getUserFeed().json(fetch);
    
    return {
        entries
    }

}
