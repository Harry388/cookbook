import { getCommunity, getCommunityPosts } from '$lib/app/community';

export const load = async ({ params, fetch }) => {

    const community = await getCommunity(params.id, fetch);

    const posts = await getCommunityPosts(params.id, fetch);

    return {
        community,
        posts,
        title: community.title
    }
}

