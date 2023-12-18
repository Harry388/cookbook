import { getPost, getPostRecipes } from '$lib/app/post';

export const load = async ({ params, fetch, parent }) => {

    const { id } = await parent();

    const [post, recipes] = await Promise.all([getPost(params.id, fetch), getPostRecipes(params.id, fetch)]);
    
    return {
        post,
        recipes,
        title: post.title,
        ownsPost: id == post.user_id
    }

}