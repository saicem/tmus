import { useEffect } from "react"
import { fetchUpdate } from "@/lib/cmd"
import { useAppStore } from "@/lib/state"

/**
 * Initializes app state (config, theme, autostart) once on mount, checks for
 * updates, and periodically emits a refresh event consumed by pages.
 */
export function AppInit() {
  useEffect(() => {
    void (async () => {
      await useAppStore.getState().init()
      const { config, showUpdateDialog } = useAppStore.getState()
      if (config.autoCheckUpdate) {
        const update = await fetchUpdate()
        if (update) showUpdateDialog(update)
      }
    })()
    const timer = window.setInterval(() => {
      window.dispatchEvent(new Event("tmus:refresh"))
    }, 60_000)
    return () => window.clearInterval(timer)
  }, [])
  return null
}
