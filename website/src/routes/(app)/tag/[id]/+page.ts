import { getTagEntries, getTag } from '$lib/app/tag';

export const load = async ({ fetch, params }) => {

    const [
        entries,
        tag
    ] = await Promise.all([
        getTagEntries(params.id).json(fetch),
        getTag(params.id).json(fetch) 
    ]);

    return {
        entries,
        title: tag.tag
    }

}
