import { getUserPosts } from '$lib/app/post';

export const load = async ({ params, fetch }) => {
    
    const posts = getUserPosts(params.id, fetch);

    return {
        posts
    }
}