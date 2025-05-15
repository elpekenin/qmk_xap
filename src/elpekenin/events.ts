import mitt, { Emitter } from 'mitt'

type UserEvent = {
    housekeeping: { time: number };
}

export const elpekenin_events: Emitter<UserEvent> = mitt<UserEvent>()
