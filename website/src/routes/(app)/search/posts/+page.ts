import { searchPosts } from '$lib/app/search';

export const load = async ({ parent, fetch }) => {

    const { search } = await parent();

    const posts = await search ? searchPosts(search).json(fetch) : '';

    return {
        posts
    }
    
}
