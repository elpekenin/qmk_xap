// TODO: move this into the plugin, and perhaps publish to npm

import { invoke } from '@tauri-apps/api/core'

type Result<T, E> =
    | {status: "ok", data: T}
    | {status: "error", error: E}

export async function get(key: string): Promise<Result<string, string>> {
    try {
      return {status: "ok", data: await invoke('plugin:env|get', { key }) }
    } catch (e) {
      return {status: "error", error: e as any}
    }
}
