//@ts-ignore
import wasm from '../pkg/wasm_bg.wasm?module'
import init, { entry_point } from '../pkg/wasm.js'

export const config = {
    runtime: 'edge',
}

export default async function handler(request: Request) {
    //[[IF DATABASE Planetscale]]
    // @ts-ignore - Idk why but this is needed if you want to use the env vars
    const { PS_HOST, PS_USER, PS_PASS } = process.env;
    //[[ENDIF]]

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

    let url = new URL(request.url);
    url.searchParams.delete('rew');

    // @ts-ignore
    let query = new Map<string, string>();
    // @ts-ignore
    for (const [key, value] of url.searchParams.entries()) {
        query.set(key, value);
    }

    await init(wasm);
    let response: WasmResponse = await entry_point({
        url: url.pathname,
        method: request.method,
        headers: headers,
        body: body,
        query: query,

        // @ts-ignore
        params: new Map<string, string>(),

        // @ts-ignore
        env: process.env as any
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
    body: number[];

    // @ts-ignore
    params: Map<string, string>;

    // @ts-ignore
    query: Map<string, string>;

    // @ts-ignore
    env: Map<string, string>;
};

export type WasmResponse = {
    status: number;
    headers: [string, string][];
    body: number[];
};
