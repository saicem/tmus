import { useCallback, useEffect, useMemo, useRef, useState } from "react"
import { useNavigate } from "react-router-dom"
import {
  columnFilteringFeature,
  columnVisibilityFeature,
  createColumnHelper,
  createFilteredRowModel,
  createSortedRowModel,
  filterFn_includesString,
  FlexRender,
  globalFilteringFeature,
  rowSortingFeature,
  sortFn_alphanumeric,
  tableFeatures,
  useTable,
  type Column,
  type ColumnFiltersState,
  type SortingState,
} from "@tanstack/react-table"
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
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { AppIcon } from "@/components/common/AppIcon"
import { writeText } from "@tauri-apps/plugin-clipboard-manager"
import { toast } from "sonner"
import { getAppDetailMap, showInFolder } from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { FileDetail } from "@/lib/models"
import { useAppStore } from "@/lib/state"
import { cn } from "@/lib/utils"

const features = tableFeatures({
  columnFilteringFeature,
  globalFilteringFeature,
  columnVisibilityFeature,
  rowSortingFeature,
  filteredRowModel: createFilteredRowModel(),
  sortedRowModel: createSortedRowModel(),
  filterFns: { includesString: filterFn_includesString },
  sortFns: { alphanumeric: sortFn_alphanumeric },
})

const columnHelper = createColumnHelper<typeof features, FileDetail>()

function TruncatedText({
  text,
  className,
}: {
  text: string
  className?: string
}) {
  const ref = useRef<HTMLSpanElement>(null)
  const [truncated, setTruncated] = useState(false)

  useEffect(() => {
    const el = ref.current
    if (!el) return
    const update = () => setTruncated(el.scrollWidth > el.clientWidth + 1)
    update()
    const observer = new ResizeObserver(update)
    observer.observe(el)
    return () => observer.disconnect()
  }, [text])

  const content = (
    <span ref={ref} className={cn("truncate", className)}>
      {text}
    </span>
  )

  if (!truncated) return content

  return (
    <Tooltip>
      <TooltipTrigger asChild>{content}</TooltipTrigger>
      <TooltipContent className="break-words">{text}</TooltipContent>
    </Tooltip>
  )
}

function SortableHeader<TValue>({
  column,
  title,
}: {
  column: Column<typeof features, FileDetail, TValue>
  title: string
}) {
  const sorted = column.getIsSorted()
  return (
    <Button
      variant="ghost"
      size="sm"
      className="-ml-3 h-7 max-w-full px-2 font-medium"
      onClick={() => column.toggleSorting(sorted === "asc")}
    >
      <TruncatedText text={title} className="min-w-0" />
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

export function Application() {
  const t = useI18n()
  const navigate = useNavigate()
  const filterUninstalled = useAppStore((s) => s.config.filterUninstalledApp)
  const [apps, setApps] = useState<FileDetail[]>([])
  const [sorting, setSorting] = useState<SortingState>([])
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([])
  const [globalFilter, setGlobalFilter] = useState("")
  const [columnVisibility, setColumnVisibility] = useState({})
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
  }, [updateScrollShadows, apps])

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

  const columns = useMemo(
    () =>
      columnHelper.columns([
        columnHelper.accessor("name", {
          id: "name",
          header: ({ column }) => (
            <SortableHeader
              column={column}
              title={t.navigateMenu.application}
            />
          ),
          cell: ({ row }) => {
            const app = row.original
            return (
              <span className="flex min-w-0 items-center gap-2">
                <AppIcon icon={app.icon} exist={app.exist} size={16} />
                <TruncatedText
                  text={app.name}
                  className={
                    app.exist ? "" : "text-muted-foreground line-through"
                  }
                />
              </span>
            )
          },
        }),
        columnHelper.accessor((row) => row.version?.companyName ?? "", {
          id: "company",
          header: ({ column }) => (
            <SortableHeader column={column} title={t.applicationPage.company} />
          ),
          cell: ({ row }) => (
            <TruncatedText
              text={row.original.version?.companyName ?? "-"}
              className="block text-muted-foreground"
            />
          ),
        }),
        columnHelper.accessor("path", {
          id: "path",
          header: ({ column }) => (
            <SortableHeader column={column} title={t.applicationPage.path} />
          ),
          cell: ({ row }) => (
            <TruncatedText
              text={row.original.path}
              className="block text-muted-foreground"
            />
          ),
        }),
        columnHelper.display({
          id: "actions",
          enableSorting: false,
          enableHiding: false,
          cell: ({ row }) => {
            const app = row.original
            return (
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" className="size-8 p-0">
                    <span className="sr-only">
                      {t.applicationPage.openMenu}
                    </span>
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
                  <DropdownMenuItem
                    onClick={async () => {
                      try {
                        await writeText(app.path)
                        toast.success(t.applicationPage.copyPathSuccess)
                      } catch {
                        toast.error(t.applicationPage.copyPathError)
                      }
                    }}
                  >
                    {t.applicationPage.copyPath}
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
        }),
      ]),
    [t, navigate]
  )

  const table = useTable({
    features,
    data: apps,
    columns,
    state: { sorting, columnFilters, globalFilter, columnVisibility },
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    onGlobalFilterChange: setGlobalFilter,
    onColumnVisibilityChange: setColumnVisibility,
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
                  {headerGroup.headers.map((header) => (
                    <TableHead
                      key={header.id}
                      className="sticky top-0 z-10 bg-background"
                    >
                      <FlexRender header={header} />
                    </TableHead>
                  ))}
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
                      <TableCell key={cell.id}>
                        <FlexRender cell={cell} />
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
