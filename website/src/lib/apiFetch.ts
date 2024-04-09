import { PUBLIC_API_URL } from '$env/static/public';
import { error } from '$lib/app/error';

export type FetchFn = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>

type FetchObj<T> = {
    input: string, 
    body?: any, 
    init?: RequestInit,
    run: (fetch?: FetchFn) => Promise<Response>,
    json: (fetch?: FetchFn) => Promise<T>
}

function createFetchObj<T>(input: string, body?: any, init?: RequestInit): FetchObj<T> {
    return { input, body, init,
        async run(fetchFn?: FetchFn) {
            if (!fetchFn) {
                fetchFn = fetch;
            }
            //@ts-ignore
            const stringifyBody = (!this.init?.headers) || (!this.init?.headers['Content-Type']);
            const body = this.body ? (stringifyBody ? JSON.stringify(this.body) : this.body) : null;
            const headers = {
                'Accept': '*',
                'Content-Type': 'application/json',
                ...this.init?.headers,
            };
            if (headers['Content-Type'] == 'remove') {
                //@ts-ignore
                delete headers['Content-Type'];
            }
            const requestInit: RequestInit = {
                ...init,
                body,
                headers,
                credentials: 'include'
            }
            const response = await fetchFn(`${PUBLIC_API_URL}/${this.input}`, requestInit);
            if (!response.ok) {
                error.set(response.status == 512 ? await response.text() : response.statusText);
                setTimeout(() => error.set(''), 5000);
            }
            return response;
        },
        async json(fetchFn?: FetchFn) {
            const response = await this.run(fetchFn);
            const obj: T = await response.json();
            return obj;
        }
    }
}

export function get<T>(input: string, body?: any, init?: RequestInit): FetchObj<T> {
    return createFetchObj(input, body, init);
}

export function post<T>(input: string, body?: any, init?: RequestInit): FetchObj<T> {
    return createFetchObj(input, body, {
        ...init,
        method: 'post'
    })
}

export function put<T>(input: string, body?: any, init?: RequestInit): FetchObj<T> {
    return createFetchObj(input, body, {
        ...init,
        method: 'put'
    })
}

export function remove<T>(input: string, body?: any, init?: RequestInit): FetchObj<T> {
    return createFetchObj(input, body, {
        ...init,
        method: 'delete'
    })
}
