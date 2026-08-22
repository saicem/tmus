import { useEffect, useMemo, useState } from "react"
import {
  CheckIcon,
  ChevronDown,
  ChevronRight,
  ChevronsUpDown,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { Separator } from "@/components/ui/separator"
import { cn } from "@/lib/utils"
import { useI18n } from "@/lib/i18n"
import type { Category } from "@/lib/models"

function flattenCategories(categories: Category[]): Category[] {
  const result: Category[] = []
  const visit = (nodes: Category[]) => {
    for (const node of nodes) {
      result.push(node)
      visit(node.children)
    }
  }
  visit(categories)
  return result
}

function collectExpandableIds(categories: Category[]): number[] {
  const ids: number[] = []
  const visit = (nodes: Category[]) => {
    for (const node of nodes) {
      if (node.children.length > 0) ids.push(node.id)
      visit(node.children)
    }
  }
  visit(categories)
  return ids
}

interface TreeRowProps {
  node: Category
  depth: number
  multiple: boolean
  isChecked: (id: number) => boolean
  isSelected: (id: number) => boolean
  isExpanded: (id: number) => boolean
  onToggleExpand: (id: number) => void
  onPick: (id: number) => void
}

function TreeRow({
  node,
  depth,
  multiple,
  isChecked,
  isSelected,
  isExpanded,
  onToggleExpand,
  onPick,
}: TreeRowProps) {
  const checked = isChecked(node.id)
  const selected = isSelected(node.id)
  const expanded = isExpanded(node.id)

  return (
    <>
      <div
        className={cn(
          "flex cursor-pointer items-center gap-1.5 rounded-md py-1.5 pr-2 text-sm hover:bg-accent",
          selected && !multiple && "bg-accent"
        )}
        style={{ paddingLeft: 8 + depth * 16 }}
        onClick={() => onPick(node.id)}
      >
        {node.children.length > 0 ? (
          <button
            type="button"
            className="flex size-4 shrink-0 items-center justify-center rounded hover:bg-muted"
            onClick={(e) => {
              e.stopPropagation()
              onToggleExpand(node.id)
            }}
          >
            {expanded ? (
              <ChevronDown className="size-4" />
            ) : (
              <ChevronRight className="size-4" />
            )}
          </button>
        ) : (
          <span className="size-4 shrink-0" />
        )}
        {multiple && (
          <Checkbox
            checked={checked}
            onClick={(e) => e.stopPropagation()}
            onCheckedChange={() => onPick(node.id)}
          />
        )}
        <span className="flex-1 truncate">{node.name}</span>
        {!multiple && selected && (
          <CheckIcon className="size-4 shrink-0 text-primary" />
        )}
      </div>
      {expanded &&
        node.children.map((child) => (
          <TreeRow
            key={child.id}
            node={child}
            depth={depth + 1}
            multiple={multiple}
            isChecked={isChecked}
            isSelected={isSelected}
            isExpanded={isExpanded}
            onToggleExpand={onToggleExpand}
            onPick={onPick}
          />
        ))}
    </>
  )
}

/**
 * Category picker rendered as a tree inside a popover. Supports single-select
 * (with an "All Categories" reset row) and multi-select (with checkboxes and a
 * clear button), reflecting the hierarchical category structure.
 */
export function CategoryTreeSelect({
  categories,
  multiple = false,
  value,
  onChange,
  placeholder,
  triggerClassName,
}: {
  categories: Category[]
  multiple?: boolean
  value: string | string[]
  onChange: (value: string | string[]) => void
  placeholder: string
  triggerClassName?: string
}) {
  const t = useI18n()
  const flat = useMemo(() => flattenCategories(categories), [categories])
  const [expanded, setExpanded] = useState<Set<number>>(
    () => new Set(collectExpandableIds(categories))
  )

  useEffect(() => {
    setExpanded((prev) => {
      const next = new Set(prev)
      for (const id of collectExpandableIds(categories)) next.add(id)
      return next
    })
  }, [categories])

  const toggleExpand = (id: number) => {
    setExpanded((prev) => {
      const next = new Set(prev)
      if (next.has(id)) {
        next.delete(id)
      } else {
        next.add(id)
      }
      return next
    })
  }

  const pick = (id: number) => {
    if (multiple) {
      const current = value as string[]
      const next = current.includes(String(id))
        ? current.filter((v) => v !== String(id))
        : [...current, String(id)]
      onChange(next)
    } else {
      onChange(String(id))
    }
  }

  const selectedNames = multiple
    ? flat
        .filter((c) => (value as string[]).includes(String(c.id)))
        .map((c) => c.name)
    : value === ""
    ? []
    : flat.filter((c) => String(c.id) === value).map((c) => c.name)

  let label = placeholder
  if (selectedNames.length === 1) {
    label = selectedNames[0]
  } else if (selectedNames.length > 1) {
    label = selectedNames.slice(0, 2).join(", ")
    if (selectedNames.length > 2) {
      label = `${label} ${t.statisticPage.categories.more.replace(
        "{count}",
        String(selectedNames.length - 2)
      )}`
    }
  }

  const hasSelection = multiple ? (value as string[]).length > 0 : value !== ""
  const isChecked = (id: number) =>
    multiple && (value as string[]).includes(String(id))
  const isSelected = (id: number) => !multiple && value === String(id)
  const isExpanded = (id: number) => expanded.has(id)

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          className={cn(
            "h-9 justify-start gap-2 font-normal",
            triggerClassName
          )}
        >
          <span className="flex-1 truncate text-left">{label}</span>
          <ChevronsUpDown data-icon="inline-end" />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-80 p-0">
        <div className="max-h-[min(18rem,50vh)] overflow-x-hidden overflow-y-auto">
          <div className="flex min-w-0 flex-col gap-0.5 p-2">
            {!multiple && (
              <div
                className={cn(
                  "flex cursor-pointer items-center gap-1.5 rounded-md py-1.5 pr-2 pl-2 text-sm hover:bg-accent",
                  value === "" && "bg-accent"
                )}
                onClick={() => onChange("")}
              >
                <span className="size-4 shrink-0" />
                <span className="flex-1 truncate">
                  {t.statisticPage.categories.all}
                </span>
                {value === "" && (
                  <CheckIcon className="size-4 shrink-0 text-primary" />
                )}
              </div>
            )}
            {categories.map((node) => (
              <TreeRow
                key={node.id}
                node={node}
                depth={0}
                multiple={multiple}
                isChecked={isChecked}
                isSelected={isSelected}
                isExpanded={isExpanded}
                onToggleExpand={toggleExpand}
                onPick={pick}
              />
            ))}
          </div>
        </div>
        {multiple && hasSelection && (
          <>
            <Separator />
            <div className="flex justify-end p-2">
              <Button variant="ghost" size="sm" onClick={() => onChange([])}>
                {t.statisticPage.categories.clear}
              </Button>
            </div>
          </>
        )}
      </PopoverContent>
    </Popover>
  )
}
