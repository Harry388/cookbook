import { PUBLIC_API_URL } from '$env/static/public';

function myFetch(input: string, body?: any | null, init?: RequestInit | null): Promise<Response> {
    return fetch(`${PUBLIC_API_URL}/${input}`, {
        ...init,
        body: body ? JSON.stringify(body) : null,
        headers: {
            ...init?.headers,
            'Accept': '*',
            'Content-Type': 'application/json'
        },
        credentials: 'include'
    })
}

export const get = myFetch;

export function post(input: string, body?: any | null, init?: RequestInit | null): Promise<Response> {
    return myFetch(input, body, {
        ...init,
        method: 'post'
    })
}

export function put(input: string, body?: any | null, init?: RequestInit | null): Promise<Response> {
    return myFetch(input, body, {
        ...init,
        method: 'put'
    })
}

export function remove(input: string, body?: any | null, init?: RequestInit | null): Promise<Response> {
    return myFetch(input, body, {
        ...init,
        method: 'delete'
    })
}