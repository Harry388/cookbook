import { get } from '$lib/apiFetch';
import type { Post } from '$lib/app/post';

export const load = async ({ params, fetch }) => {
    
    const response = await get(`post/user/${params.id}`).run(fetch);

    const posts: Post[] = await response.json();

    return {
        posts
    }
}