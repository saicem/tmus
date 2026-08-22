import { NavLink } from "react-router-dom"
import {
  AppWindow,
  BarChart3,
  Clock,
  FolderTree,
  Home,
  Settings,
} from "lucide-react"
import { useI18n } from "@/lib/i18n"
import { cn } from "@/lib/util"

const items = [
  { to: "/", key: "overview", icon: Home },
  { to: "/timeline", key: "timeline", icon: Clock },
  { to: "/statistic", key: "statistic", icon: BarChart3 },
  { to: "/application", key: "application", icon: AppWindow },
  { to: "/category", key: "category", icon: FolderTree },
  { to: "/setting", key: "setting", icon: Settings },
] as const

export function Sidebar() {
  const t = useI18n()
  return (
    <nav className="flex w-48 shrink-0 flex-col gap-1 border-r bg-muted/30 p-2">
      {items.map(({ to, key, icon: Icon }) => (
        <NavLink
          key={to}
          to={to}
          end={to === "/"}
          className={({ isActive }) =>
            cn(
              "flex items-center gap-2 rounded-md px-3 py-2 text-sm text-muted-foreground transition-colors",
              "hover:bg-accent hover:text-accent-foreground",
              isActive && "bg-accent text-accent-foreground"
            )
          }
        >
          <Icon className="size-4" />
          {t.navigateMenu[key]}
        </NavLink>
      ))}
    </nav>
  )
}
