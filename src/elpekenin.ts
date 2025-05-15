import { elpekenin_events } from "@/elpekenin/events"
import * as stats from "@/elpekenin/stats"

export async function onInit() {
    stats.onInit()

    // kick-off sending of events
    setInterval(
        () => elpekenin_events.emit("housekeeping", {time: Date.now()}),
        500,
    )
}
