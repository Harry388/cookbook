import { getTrending } from '$lib/app/user';

export const load = async ({ fetch }) => {

    const entries = await getTrending().json(fetch);
    
    return {
        entries
    }

}
