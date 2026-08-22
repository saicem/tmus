import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { Maximize2, Minus, X } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useI18n } from "@/lib/i18n"

const appWindow = getCurrentWebviewWindow()

export function TitleBar() {
  const t = useI18n()
  return (
    <div
      data-tauri-drag-region
      className="flex h-8 shrink-0 select-none items-center justify-between border-b bg-background"
    >
      <span />
      <div className="flex">
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-10 rounded-none text-muted-foreground hover:bg-accent hover:text-accent-foreground"
          onClick={() => appWindow.minimize()}
          aria-label={t.common.minimize}
        >
          <Minus />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-10 rounded-none text-muted-foreground hover:bg-accent hover:text-accent-foreground"
          onClick={() => appWindow.toggleMaximize()}
          aria-label={t.common.maximize}
        >
          <Maximize2 />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-10 rounded-none text-muted-foreground hover:bg-destructive hover:text-destructive-foreground"
          onClick={() => appWindow.close()}
          aria-label={t.common.close}
        >
          <X />
        </Button>
      </div>
    </div>
  )
}
