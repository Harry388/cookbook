import { PUBLIC_BROWSER_API_URL, PUBLIC_SERVER_API_URL } from '$env/static/public';
import { browser } from '$app/environment';

type FetchFn = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>

type FetchObj = {
    input: string, 
    body?: any, 
    init?: RequestInit,
    run: (fetch?: FetchFn) => Promise<Response>
}

function createFetchObj(input: string, body?: any, init?: RequestInit): FetchObj {
    return { input, body, init,
        run(fetchFn?: FetchFn) {
            if (!fetchFn) {
                fetchFn = fetch;
            }
            const url = browser ? PUBLIC_BROWSER_API_URL : PUBLIC_SERVER_API_URL;
            const requestInit: RequestInit = {
                ...init,
                body: this.body ? JSON.stringify(this.body) : null,
                headers: {
                    ...this.init?.headers,
                    'Accept': '*',
                    'Content-Type': 'application/json',
                },
                credentials: 'include'
            }
            return fetchFn(`${url}/${this.input}`, requestInit)
        }
    }
}

export function get(input: string, body?: any, init?: RequestInit): FetchObj {
    return createFetchObj(input, body, init);
}

export function post(input: string, body?: any, init?: RequestInit): FetchObj {
    return createFetchObj(input, body, {
        ...init,
        method: 'post'
    })
}

export function put(input: string, body?: any, init?: RequestInit): FetchObj {
    return createFetchObj(input, body, {
        ...init,
        method: 'put'
    })
}

export function remove(input: string, body?: any, init?: RequestInit): FetchObj {
    return createFetchObj(input, body, {
        ...init,
        method: 'delete'
    })
}