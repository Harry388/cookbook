import { getCommunity, getCommunityPosts } from '$lib/app/community';

export const load = async ({ params, fetch }) => {

    const [community, posts] = await Promise.all([getCommunity(params.id, fetch), getCommunityPosts(params.id, fetch)]);

    return {
        community,
        posts,
        title: community.title
    }
}

