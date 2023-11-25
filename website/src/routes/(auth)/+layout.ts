export const load = ({ url }) => {
    return {
        redirect: url.searchParams.get('redirect')
    }
}