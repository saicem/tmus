import { AppWindow, FileX } from "lucide-react"
import { cn } from "@/lib/util"

/**
 * Application icon with fallbacks:
 * - real icon when available
 * - generic app icon when the icon is missing
 * - deleted-file icon when the icon is missing and the app was uninstalled
 */
export function AppIcon({
  icon,
  exist = true,
  size = 16,
  className,
}: {
  icon?: string | null
  exist?: boolean
  size?: number
  className?: string
}) {
  if (icon) {
    return (
      <img
        src={icon}
        alt=""
        className={className}
        style={{ width: size, height: size }}
      />
    )
  }
  const Icon = exist ? AppWindow : FileX
  return (
    <Icon
      aria-hidden
      className={cn("shrink-0 text-muted-foreground", className)}
      style={{ width: size, height: size }}
    />
  )
}
