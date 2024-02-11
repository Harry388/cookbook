import { getPost, getPostRecipes } from '$lib/app/post';

export const load = async ({ params, fetch, parent, depends }) => {

    const { id } = await parent();

    const [post, recipes] = await Promise.all([getPost(params.id).json(fetch), getPostRecipes(params.id).json(fetch)]);

    depends('app:post');
    
    return {
        post,
        recipes,
        title: post.title,
        ownsPost: id == post.user_id
    }

}
