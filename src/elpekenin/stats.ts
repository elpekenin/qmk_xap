import { storeToRefs } from "pinia"
import * as system_info from "tauri-plugin-system-info-api"

import { useXapDeviceStore } from '@/utils/deviceStore'
import * as xap from "@generated/xap"

import {elpekenin_events} from "@/elpekenin/events"

export function onInit() {
    elpekenin_events.on(
        "housekeeping",
        (ev) => handler(ev.time),
    )
}

async function handler(_: number) {
    await system_info.refreshAll()
    const cpu_info = await system_info.cpuInfo()
    const memory_info = await system_info.memoryInfo()

    const cpu_usage = cpu_info.cpus.reduce((sum, x) => sum + x.cpu_usage, 0) / cpu_info.cpu_count
    const memory_usage = memory_info.used_memory / memory_info.total_memory * 100

    const store = useXapDeviceStore()
    const { device } = storeToRefs(store) as { device: Ref<xap.XapDeviceState | null> }

    const ret = await xap.commands.quantumPainterpushComputerStats(
        device.value?.id!,
        {
            cpu: Math.round(cpu_usage),
            ram: Math.round(memory_usage),
        },
    )

    if (ret.status == "error") {
        console.error("Sending stats failed: ", ret.error)
    }
}
