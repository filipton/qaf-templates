//@ts-ignore
import wasm from '../pkg/wasm_bg.wasm?module'
import init, { entry_point } from '../pkg/wasm.js'

export const config = {
    runtime: 'edge',
}

export default async function handler(request: Request) {
    // @ts-ignore
    const { LIBSQL_CLIENT_URL, LIBSQL_CLIENT_TOKEN } = process.env;

    let body: number[] = [];
    if (request.body) {
        const reader = request.body.getReader();
        let result = await reader.read();
        let tmp: [Uint8Array] = [new Uint8Array(0)];

        while (!result.done) {
            tmp.push(result.value);
            result = await reader.read();
        }

        for (let i = 0; i < tmp.length; i++) {
            // @ts-ignore
            body = body.concat(Array.from(tmp[i]));
        }
    }

    // @ts-ignore
    const headers = Array.from(request.headers.entries());

    const rewIndexOf = request.url.indexOf('rew=');
    let rewEnd = request.url.indexOf('&', rewIndexOf);
    if (rewEnd === -1) {
        rewEnd = request.url.length;
    }

    let url = request.url.substring(0, rewIndexOf) +
        request.url.substring(rewEnd, request.url.length);
    if (url[url.length - 1] === '?' || url[url.length - 1] === '&') {
        url = url.substring(0, url.length - 1);
    }

    await init(wasm);
    let response: WasmResponse = await entry_point({
        url: url,
        method: request.method,
        headers: headers,

        // @ts-ignore
        env: process.env as any,
        body: body
    } as WasmRequest);

    let respBody = new Uint8Array(response.body);

    return new Response(respBody, {
        status: response.status,
        headers: response.headers
    });
}

export type WasmRequest = {
    url: string;
    method: string;
    headers: [string, string][];

    // @ts-ignore
    env: Map<string, string>;
    body: number[];
};

export type WasmResponse = {
    status: number;
    headers: [string, string][];
    body: number[];
};
