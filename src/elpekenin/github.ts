import * as octokit from "@octokit/core"

import * as xap from "@generated/xap"

import * as elpekenin from "@/elpekenin"
import * as env from "@/elpekenin/env"

export function onInit() {
    handler()
    setInterval(handler, elpekenin.Seconds(30))
}

async function handler() {
    const token = await env.get("GITHUB_TOKEN")
    if (token.status == "error") {
        console.error("Could not get GITHUB_TOKEN from env: ", token.error)
        return
    }

    const client = new octokit.Octokit({
        auth: token.data,
    })

    const response = await client.request('GET /notifications', {})
    if (response.status != 200) {
        console.error("GitHub API returned an error status %d", response.status)
        return
    }

    const ret = await xap.commands.taskssetGithubNotificationsCount(
        elpekenin.getDeviceId()!,
        {
            count: response.data.length
        }
    )

    if (ret.status == "error") {
        console.error("Setting GitHub notification count failed: ", ret.error)
    }
}