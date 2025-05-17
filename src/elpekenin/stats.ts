import * as system_info from "tauri-plugin-system-info-api"

import * as xap from "@generated/xap"

import * as elpekenin from "@/elpekenin"

export function onInit() {
    setInterval(handler, elpekenin.Seconds(1))
}

async function handler() {
    await system_info.refreshAll()
    const cpu_info = await system_info.cpuInfo()
    const memory_info = await system_info.memoryInfo()

    const cpu_usage = cpu_info.cpus.reduce((sum, x) => sum + x.cpu_usage, 0) / cpu_info.cpu_count
    const memory_usage = memory_info.used_memory / memory_info.total_memory * 100

    const ret = await xap.commands.taskspushComputerStats(
        elpekenin.getDeviceId()!,
        {
            cpu: Math.round(cpu_usage),
            ram: Math.round(memory_usage),
        },
    )

    if (ret.status == "error") {
        console.error("Sending stats failed: ", ret.error)
    }
}
