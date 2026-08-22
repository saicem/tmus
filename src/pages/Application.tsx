import { useCallback, useEffect, useMemo, useRef, useState } from "react"
import { useNavigate } from "react-router-dom"
import {
  flexRender,
  type ColumnFiltersState,
  type SortingState,
} from "@tanstack/react-table"
import {
  getCoreRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  useLegacyTable,
  type LegacyColumn,
  type LegacyColumnDef,
} from "@tanstack/react-table/legacy"
import {
  ArrowDown,
  ArrowUp,
  ArrowUpDown,
  MoreHorizontal,
  Search,
  Settings2,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Input } from "@/components/ui/input"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { AppIcon } from "@/components/common/AppIcon"
import { getAppDetailMap, showInFolder } from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { FileDetail } from "@/lib/models"
import { useAppStore } from "@/lib/state"

const DEFAULT_COLUMN_WIDTHS: Record<string, number> = {
  name: 120,
  company: 90,
  path: 320,
  actions: 110,
}

function SortableHeader({
  column,
  title,
}: {
  column: LegacyColumn<FileDetail, unknown>
  title: string
}) {
  const sorted = column.getIsSorted()
  return (
    <Button
      variant="ghost"
      size="sm"
      className="-ml-3 h-7 px-2 font-medium"
      onClick={() => column.toggleSorting(sorted === "asc")}
    >
      {title}
      {sorted === "asc" ? (
        <ArrowUp className="ml-1 size-3" />
      ) : sorted === "desc" ? (
        <ArrowDown className="ml-1 size-3" />
      ) : (
        <ArrowUpDown className="ml-1 size-3" />
      )}
    </Button>
  )
}

function ColumnResizer({
  width,
  onWidth,
  title,
}: {
  width: number
  onWidth: (w: number) => void
  title: string
}) {
  const handleMouseDown = (e: React.MouseEvent<HTMLDivElement>) => {
    e.preventDefault()
    e.stopPropagation()
    const startX = e.clientX
    const onMove = (ev: MouseEvent) => {
      onWidth(Math.max(60, width + ev.clientX - startX))
    }
    const onUp = () => {
      window.removeEventListener("mousemove", onMove)
      window.removeEventListener("mouseup", onUp)
    }
    window.addEventListener("mousemove", onMove)
    window.addEventListener("mouseup", onUp)
  }
  return (
    <div
      onMouseDown={handleMouseDown}
      className="absolute inset-y-0 right-0 w-1 cursor-col-resize bg-transparent transition-colors hover:bg-primary"
      title={title}
    />
  )
}

export function Application() {
  const t = useI18n()
  const navigate = useNavigate()
  const filterUninstalled = useAppStore((s) => s.config.filterUninstalledApp)
  const [apps, setApps] = useState<FileDetail[]>([])
  const [sorting, setSorting] = useState<SortingState>([])
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([])
  const [globalFilter, setGlobalFilter] = useState("")
  const [columnVisibility, setColumnVisibility] = useState({})
  const [columnWidths, setColumnWidths] = useState<Record<string, number>>(
    DEFAULT_COLUMN_WIDTHS
  )
  const tableScrollRef = useRef<HTMLDivElement>(null)
  const [showLeftShadow, setShowLeftShadow] = useState(false)
  const [showRightShadow, setShowRightShadow] = useState(false)

  const updateScrollShadows = useCallback(() => {
    const el = tableScrollRef.current
    if (!el) return
    setShowLeftShadow(el.scrollLeft > 2)
    setShowRightShadow(el.scrollLeft + el.clientWidth < el.scrollWidth - 2)
  }, [])

  useEffect(() => {
    const el = tableScrollRef.current
    if (!el) return
    updateScrollShadows()
    const observer = new ResizeObserver(updateScrollShadows)
    observer.observe(el)
    const tableEl = el.querySelector("table")
    if (tableEl) observer.observe(tableEl)
    return () => observer.disconnect()
  }, [updateScrollShadows, apps, columnWidths])

  useEffect(() => {
    let cancelled = false
    void (async () => {
      let result = Object.values(await getAppDetailMap())
      if (filterUninstalled) {
        result = result.filter((app) => app.exist)
      }
      result.sort((a, b) => a.name.localeCompare(b.name))
      if (!cancelled) setApps(result)
    })()
    return () => {
      cancelled = true
    }
  }, [filterUninstalled])

  const openDetail = (id: number) => navigate(`/detail/${id}`)

  const setColumnWidth = (id: string) => (width: number) =>
    setColumnWidths((prev) => ({ ...prev, [id]: width }))

  const columns = useMemo<LegacyColumnDef<FileDetail>[]>(
    () => [
      {
        id: "name",
        accessorKey: "name",
        header: ({ column }) => (
          <SortableHeader column={column} title={t.navigateMenu.application} />
        ),
        cell: ({ row }) => {
          const app = row.original
          return (
            <span className="flex min-w-0 items-center gap-2">
              <AppIcon icon={app.icon} exist={app.exist} size={16} />
              <span
                className={`truncate ${
                  app.exist ? "" : "text-muted-foreground line-through"
                }`}
              >
                {app.name}
              </span>
            </span>
          )
        },
      },
      {
        id: "company",
        accessorFn: (row) => row.version?.companyName ?? "",
        header: ({ column }) => (
          <SortableHeader column={column} title={t.applicationPage.company} />
        ),
        cell: ({ row }) => (
          <span className="text-muted-foreground">
            {row.original.version?.companyName ?? "-"}
          </span>
        ),
      },
      {
        id: "path",
        accessorKey: "path",
        header: ({ column }) => (
          <SortableHeader column={column} title={t.applicationPage.path} />
        ),
        cell: ({ row }) => (
          <span className="block max-w-80 truncate text-muted-foreground">
            {row.original.path}
          </span>
        ),
      },
      {
        id: "actions",
        enableSorting: false,
        enableHiding: false,
        cell: ({ row }) => {
          const app = row.original
          return (
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="ghost" className="size-8 p-0">
                  <span className="sr-only">{t.applicationPage.openMenu}</span>
                  <MoreHorizontal />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuLabel>
                  {t.applicationPage.operations}
                </DropdownMenuLabel>
                <DropdownMenuItem onClick={() => openDetail(app.id)}>
                  {t.applicationPage.details}
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem
                  disabled={!app.exist}
                  onClick={() => app.exist && showInFolder(app.path)}
                >
                  {t.applicationPage.showInFolder}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          )
        },
      },
    ],
    [t, navigate]
  )

  const table = useLegacyTable({
    data: apps,
    columns,
    state: { sorting, columnFilters, globalFilter, columnVisibility },
    enableColumnResizing: true,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    onGlobalFilterChange: setGlobalFilter,
    onColumnVisibilityChange: setColumnVisibility,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    globalFilterFn: "includesString",
  })

  const toggleableColumns = table
    .getAllLeafColumns()
    .filter((col) => col.getCanHide())

  return (
    <div className="flex h-full flex-col gap-3 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <div className="relative">
          <Search className="absolute left-2.5 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            value={globalFilter}
            onChange={(e) => setGlobalFilter(e.target.value)}
            placeholder={t.applicationPage.search}
            className="w-72 pl-8"
          />
        </div>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" size="sm" className="gap-2">
              <Settings2 className="size-4" />
              {t.applicationPage.columns}
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {toggleableColumns.map((col) => (
              <DropdownMenuCheckboxItem
                key={col.id}
                checked={col.getIsVisible()}
                onCheckedChange={(v) => col.toggleVisibility(!!v)}
              >
                {col.id === "name"
                  ? t.navigateMenu.application
                  : col.id === "company"
                  ? t.applicationPage.company
                  : col.id === "path"
                  ? t.applicationPage.path
                  : col.id}
              </DropdownMenuCheckboxItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
        <span className="ml-auto shrink-0 text-sm text-muted-foreground">
          {table.getFilteredRowModel().rows.length} {t.applicationPage.count}
        </span>
      </div>

      <div className="relative min-h-0 flex-1">
        <div
          ref={tableScrollRef}
          onScroll={updateScrollShadows}
          className="h-full overflow-auto rounded-md border"
        >
          <Table containerClassName="overflow-x-visible">
            <TableHeader>
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => {
                    const id = header.column.id
                    const width = columnWidths[id] ?? 150
                    return (
                      <TableHead
                        key={header.id}
                        className="sticky top-0 z-10 bg-background"
                        style={{ width }}
                      >
                        {flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                        {header.column.getCanResize() && (
                          <ColumnResizer
                            width={width}
                            onWidth={setColumnWidth(id)}
                            title={t.applicationPage.dragToResize}
                          />
                        )}
                      </TableHead>
                    )
                  })}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              {table.getRowModel().rows.length === 0 ? (
                <TableRow>
                  <TableCell
                    colSpan={columns.length}
                    className="h-24 text-center text-muted-foreground"
                  >
                    {t.statisticPage.validation.noData}
                  </TableCell>
                </TableRow>
              ) : (
                table.getRowModel().rows.map((row) => (
                  <TableRow key={row.id}>
                    {row.getVisibleCells().map((cell) => (
                      <TableCell
                        key={cell.id}
                        style={{ width: columnWidths[cell.column.id] ?? 150 }}
                      >
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </TableCell>
                    ))}
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </div>
        {showLeftShadow ? (
          <div className="pointer-events-none absolute inset-y-0 left-0 w-8 rounded-l-md bg-gradient-to-r from-background to-transparent" />
        ) : null}
        {showRightShadow ? (
          <div className="pointer-events-none absolute inset-y-0 right-0 w-8 rounded-r-md bg-gradient-to-l from-background to-transparent" />
        ) : null}
      </div>
    </div>
  )
}
