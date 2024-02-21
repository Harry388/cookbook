import { redirect } from '@sveltejs/kit';

export const load = async ({ route, url }) => {

    if (route.id == '/(app)/search') {
        throw redirect(301, '/search/posts');
    }

    let search = url.searchParams.get('s');

    return {
        search,
        title: search,
        path: route.id.split('/').slice(-1)[0] // for tabs
    }

}
