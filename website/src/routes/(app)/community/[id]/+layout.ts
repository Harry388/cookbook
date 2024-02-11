import { getCommunity, getCommunityPosts } from '$lib/app/community';

export const load = async ({ fetch, params, depends }) => {

    const [community, posts] = await Promise.all([getCommunity(params.id).json(fetch), getCommunityPosts(params.id).json(fetch)]);

    depends('app:community');

    return {
        community,
        posts,
        title: community.title
    }

}
