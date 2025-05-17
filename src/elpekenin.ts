import mitt, { Emitter } from 'mitt'
import { storeToRefs } from "pinia"

import { useXapDeviceStore } from '@/utils/deviceStore'
import { XapDeviceState } from "@generated/xap"

import * as github from "@/elpekenin/github"
import * as stats from "@/elpekenin/stats"

//
// types
//

type UserEvent = {
    housekeeping: void,
}

type ArrayLen<T, N extends number, R extends T[] = []> = R['length'] extends N ? R : ArrayLen<T, N, [T, ...R]>

type Name = ArrayLen<number, 9>

//
// globals
//

export const events: Emitter<UserEvent> = mitt<UserEvent>()

//
// functions
//

export async function onInit() {
    github.onInit()
    stats.onInit()

    // kick-off sending of events
    setInterval(() => events.emit("housekeeping"), Millis(500))
}

export function getDeviceId(): string | undefined {
    const store = useXapDeviceStore()
    const { device } = storeToRefs(store) as { device: Ref<XapDeviceState | null> }
    return device.value?.id
}

export function toName(input: string) : Name {
    var temp: Name = [0, 0, 0, 0, 0, 0, 0, 0, 0]

    for (let [index, _] of temp.entries()) {
        const char = input.charCodeAt(index)
        if (isNaN(char)) {
            break
        }

        temp[index] = char
    }

    return temp
}

export function Millis(x: number) : number {
    return x
}

export function Seconds(x: number) : number {
    return x * Millis(1000)
}

export function Minutes(x: number) : number {
    return x * Seconds(60)
}
