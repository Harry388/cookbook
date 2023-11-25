import type { PageLoad } from './$types';

export const load: PageLoad = ({ url }) => {
    return {
        redirect: url.searchParams.redirect
    }
}