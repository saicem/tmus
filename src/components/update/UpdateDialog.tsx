import { useState } from "react"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Progress } from "@/components/ui/progress"
import { installUpdate } from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { DownloadEvent } from "@/lib/models"
import { useAppStore } from "@/lib/state"

export function UpdateDialog() {
  const t = useI18n()
  const updateDialog = useAppStore((s) => s.updateDialog)
  const closeUpdateDialog = useAppStore((s) => s.closeUpdateDialog)
  const [downloading, setDownloading] = useState(false)
  const [contentLength, setContentLength] = useState(1)
  const [downloadedLength, setDownloadedLength] = useState(0)

  const confirmUpdate = async () => {
    if (downloading) return
    setDownloading(true)
    try {
      await installUpdate((event: DownloadEvent) => {
        if (event.event === "Started") {
          setContentLength(event.data.contentLength ?? 1)
          setDownloadedLength(0)
        } else if (event.event === "Progress") {
          setDownloadedLength((prev) => prev + (event.data.chunkLength ?? 0))
        } else if (event.event === "Finished") {
          setDownloading(false)
          closeUpdateDialog()
        }
      })
    } catch {
      setDownloading(false)
    }
  }

  return (
    <Dialog
      open={updateDialog.show}
      onOpenChange={(v) => !v && closeUpdateDialog()}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t.configPage.versionUpdateTitle}</DialogTitle>
        </DialogHeader>
        <p className="text-sm">
          {updateDialog.meta
            ? t.configPage.updateAvailable(
                updateDialog.meta.version,
                updateDialog.meta.currentVersion
              )
            : ""}
        </p>
        {downloading && (
          <Progress
            value={Math.round(
              (downloadedLength / Math.max(contentLength, 1)) * 100
            )}
          />
        )}
        {!downloading && (
          <DialogFooter>
            <Button variant="outline" onClick={closeUpdateDialog}>
              {t.common.cancel}
            </Button>
            <Button onClick={() => void confirmUpdate()}>{t.common.ok}</Button>
          </DialogFooter>
        )}
      </DialogContent>
    </Dialog>
  )
}
