import { queryDevice } from '@/commands/core'
import { KeyPosition } from '@bindings/KeyPosition'
import { XAPKeyCodeConfig } from '@bindings/XAPKeyCodeConfig'
import { XAPKeyInfo } from '@bindings/XAPKeyInfo'

export async function getKeycode(id: string, position: KeyPosition): Promise<number> {
    return await queryDevice('keycode_get', id, position)
}

export async function getKeyMap(id: string): Promise<Array<Array<Array<XAPKeyCodeConfig>>>> {
    return await queryDevice('keymap_get', id, null)
}

export async function getFrontEndKeyMap(id: string): Promise<Array<Array<Array<XAPKeyInfo|undefined>>>> {
    return await queryDevice('frontend_keymap_get', id, null)
}