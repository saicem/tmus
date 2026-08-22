import { useCallback, useEffect, useMemo, useState } from "react"
import { FolderPlus, Pencil, Plus, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import {
  addCategory,
  deleteCategory,
  getCategoryApps,
  getCategoryTree,
  getUncategorizedApps,
  removeAppFromCategory,
  setAppCategory,
  updateCategory,
} from "@/lib/cmd"
import { toast } from "sonner"
import { useI18n } from "@/lib/i18n"
import type { Category, CategoryId, FileDetail } from "@/lib/models"

interface CategoryOption {
  label: string
  categoryId: CategoryId
}

function buildCategoryOptions(categories: Category[]): CategoryOption[] {
  let options: CategoryOption[] = []
  for (const category of categories) {
    options.push({ label: category.name, categoryId: category.id })
    if (category.children.length > 0) {
      options = [...options, ...buildCategoryOptions(category.children)]
    }
  }
  return options
}

interface TreeNodeProps {
  node: Category
  depth: number
  onSelect: (node: Category) => void
  onAddChild: (parent: Category) => void
  onUpdate: (id: CategoryId, name: string) => void
  onDelete: (id: CategoryId) => void
}

function TreeNode({
  node,
  depth,
  onSelect,
  onAddChild,
  onUpdate,
  onDelete,
}: TreeNodeProps) {
  const t = useI18n()
  const [editing, setEditing] = useState(false)
  const [editName, setEditName] = useState(node.name)

  const saveEdit = () => {
    const name = editName.trim()
    if (!name) {
      toast.warning(t.categoryPage.categoryNameCannotBeEmpty)
      return
    }
    onUpdate(node.id, name)
    setEditing(false)
  }

  return (
    <div>
      <div
        className="group flex cursor-pointer items-center justify-between rounded-md px-2 py-1.5 hover:bg-accent"
        style={{ paddingLeft: `${8 + depth * 16}px` }}
        onClick={() => onSelect(node)}
      >
        {editing ? (
          <Input
            autoFocus
            value={editName}
            onChange={(e) => setEditName(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") saveEdit()
              if (e.key === "Escape") setEditing(false)
            }}
            onClick={(e) => e.stopPropagation()}
            className="h-7 max-w-40"
          />
        ) : (
          <span className="truncate text-sm font-medium">
            {node.name}
            <span className="ml-1 text-xs text-muted-foreground">
              ({node.appIds?.length ?? 0})
            </span>
          </span>
        )}
        {!editing && (
          <span
            className="flex shrink-0 gap-0.5 opacity-0 transition-opacity group-hover:opacity-100"
            onClick={(e) => e.stopPropagation()}
          >
            <Button
              variant="ghost"
              size="icon"
              className="size-6"
              onClick={() => onAddChild(node)}
              aria-label={t.categoryPage.addChild}
            >
              <Plus className="size-3.5" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="size-6"
              onClick={() => {
                setEditName(node.name)
                setEditing(true)
              }}
              aria-label={t.categoryPage.edit}
            >
              <Pencil className="size-3.5" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="size-6 text-destructive hover:text-destructive"
              onClick={() => onDelete(node.id)}
              aria-label={t.categoryPage.delete}
            >
              <Trash2 className="size-3.5" />
            </Button>
          </span>
        )}
      </div>
      {node.children.map((child) => (
        <TreeNode
          key={child.id}
          node={child}
          depth={depth + 1}
          onSelect={onSelect}
          onAddChild={onAddChild}
          onUpdate={onUpdate}
          onDelete={onDelete}
        />
      ))}
    </div>
  )
}

export function CategoryPage() {
  const t = useI18n()
  const [categories, setCategories] = useState<Category>({
    id: 0,
    name: "root",
    children: [],
    appIds: [],
  })
  const [uncategorized, setUncategorized] = useState<FileDetail[]>([])
  const [categoryApps, setCategoryApps] = useState<FileDetail[]>([])
  const [selected, setSelected] = useState<Category | null>(null)
  const [activeTab, setActiveTab] = useState<"uncategorized" | "categorized">(
    "uncategorized"
  )
  const [keyword, setKeyword] = useState("")
  const [addDialogOpen, setAddDialogOpen] = useState(false)
  const [addParent, setAddParent] = useState<Category | null>(null)
  const [addName, setAddName] = useState("")
  const [deleteTarget, setDeleteTarget] = useState<CategoryId | null>(null)

  const loadCategories = useCallback(async () => {
    try {
      setCategories(await getCategoryTree())
    } catch (error) {
      toast.error(t.categoryPage.failedToLoadCategories)
      console.error(error)
    }
  }, [t])

  const loadUncategorized = useCallback(async () => {
    try {
      setUncategorized(
        await getUncategorizedApps(0, 100, keyword.trim() || undefined)
      )
    } catch (error) {
      toast.error(t.categoryPage.failedToLoadUncategorizedApplications)
      console.error(error)
    }
  }, [keyword, t])

  const loadCategoryApps = useCallback(async () => {
    if (!selected) return
    try {
      setCategoryApps(await getCategoryApps(selected.id))
    } catch (error) {
      toast.error(t.categoryPage.failedToLoadApplications)
      console.error(error)
    }
  }, [selected, t])

  useEffect(() => {
    void loadCategories()
    void loadUncategorized()
  }, [loadCategories, loadUncategorized])

  const categoryOptions = useMemo(
    () => buildCategoryOptions(categories.children),
    [categories]
  )

  const openAddDialog = (parent: Category | null) => {
    setAddParent(parent)
    setAddName("")
    setAddDialogOpen(true)
  }

  const handleAddCategory = async () => {
    const name = addName.trim()
    if (!name) {
      toast.warning(t.categoryPage.categoryNameCannotBeEmpty)
      return
    }
    try {
      await addCategory({ name, parentId: addParent?.id ?? 0 })
      toast.success(t.categoryPage.categoryAddedSuccessfully)
      setAddDialogOpen(false)
      await loadCategories()
    } catch (error) {
      toast.error(t.categoryPage.failedToAddCategory)
      console.error(error)
    }
  }

  const handleUpdateCategory = async (id: CategoryId, name: string) => {
    try {
      await updateCategory({ id, name })
      toast.success(t.categoryPage.categoryUpdatedSuccessfully)
      await loadCategories()
    } catch (error) {
      toast.error(t.categoryPage.failedToUpdateCategory)
      console.error(error)
    }
  }

  const handleDeleteCategory = async (id: CategoryId) => {
    try {
      await deleteCategory({ id })
      toast.success(t.categoryPage.categoryDeletedSuccessfully)
      if (selected?.id === id) {
        setSelected(null)
        setActiveTab("uncategorized")
      }
      await loadCategories()
    } catch (error) {
      toast.error(t.categoryPage.failedToDeleteCategory)
      console.error(error)
    }
  }

  const refreshAfterAssign = async () => {
    await loadCategories()
    await loadCategoryApps()
    await loadUncategorized()
  }

  const handleAssign = async (appId: number, categoryId: CategoryId) => {
    try {
      await setAppCategory({ appId, categoryId })
      toast.success(t.categoryPage.applicationAssignedSuccessfully)
      await refreshAfterAssign()
    } catch (error) {
      toast.error(t.categoryPage.failedToAssignApplication)
      console.error(error)
    }
  }

  const handleRemove = async (appId: number) => {
    try {
      await removeAppFromCategory({ appId })
      toast.success(t.categoryPage.applicationCategoryRemovedSuccessfully)
      await refreshAfterAssign()
    } catch (error) {
      toast.error(t.categoryPage.failedToRemoveCategoryFromApplication)
      console.error(error)
    }
  }

  const handleNodeSelect = async (node: Category) => {
    setSelected(node)
    setActiveTab("categorized")
    setCategoryApps([])
    await loadCategoryApps()
  }

  return (
    <div className="flex h-full flex-col gap-4 p-4">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-semibold">{t.categoryPage.title}</h2>
        <Button onClick={() => openAddDialog(null)}>
          <FolderPlus data-icon="inline-start" />
          {t.categoryPage.addRootCategory}
        </Button>
      </div>

      <div className="flex min-h-0 flex-1 gap-4">
        <Card className="flex w-80 shrink-0 flex-col overflow-hidden">
          <CardHeader>
            <CardTitle className="text-base">
              {t.categoryPage.categoryTree}
            </CardTitle>
          </CardHeader>
          <CardContent className="min-h-0 flex-1 overflow-auto">
            {categories.children.map((child) => (
              <TreeNode
                key={child.id}
                node={child}
                depth={0}
                onSelect={handleNodeSelect}
                onAddChild={openAddDialog}
                onUpdate={handleUpdateCategory}
                onDelete={(id) => setDeleteTarget(id)}
              />
            ))}
          </CardContent>
        </Card>

        <Card className="flex min-w-0 flex-1 flex-col overflow-hidden">
          <CardContent className="flex min-h-0 flex-1 flex-col">
            <Tabs
              value={activeTab}
              onValueChange={(v) =>
                setActiveTab(v as "uncategorized" | "categorized")
              }
              className="min-h-0 flex-1"
            >
              <TabsList>
                <TabsTrigger value="uncategorized">
                  {t.categoryPage.uncategorizedApplications}
                </TabsTrigger>
                {selected && (
                  <TabsTrigger value="categorized">{selected.name}</TabsTrigger>
                )}
              </TabsList>
              <TabsContent
                value="uncategorized"
                className="flex min-h-0 flex-1 flex-col gap-3"
              >
                <Input
                  placeholder={t.categoryPage.searchPlaceholder}
                  value={keyword}
                  onChange={(e) => {
                    setKeyword(e.target.value)
                    void loadUncategorized()
                  }}
                  className="max-w-sm"
                />
                <div className="min-h-0 flex-1 overflow-auto rounded-md border">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>{t.categoryPage.applicationName}</TableHead>
                        <TableHead>{t.categoryPage.path}</TableHead>
                        <TableHead className="w-44">
                          {t.categoryPage.operation}
                        </TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {uncategorized.map((app) => (
                        <TableRow key={app.id}>
                          <TableCell>{app.name}</TableCell>
                          <TableCell className="text-muted-foreground">
                            {app.path}
                          </TableCell>
                          <TableCell>
                            <Select
                              onValueChange={(val) =>
                                handleAssign(app.id, Number(val))
                              }
                            >
                              <SelectTrigger className="h-8 w-36">
                                <SelectValue
                                  placeholder={t.categoryPage.selectCategory}
                                />
                              </SelectTrigger>
                              <SelectContent>
                                {categoryOptions.map((opt) => (
                                  <SelectItem
                                    key={opt.categoryId}
                                    value={String(opt.categoryId)}
                                  >
                                    {opt.label}
                                  </SelectItem>
                                ))}
                              </SelectContent>
                            </Select>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>
              </TabsContent>
              <TabsContent
                value="categorized"
                className="flex min-h-0 flex-1 flex-col gap-3"
              >
                <div className="min-h-0 flex-1 overflow-auto rounded-md border">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>{t.categoryPage.applicationName}</TableHead>
                        <TableHead>{t.categoryPage.path}</TableHead>
                        <TableHead className="w-44">
                          {t.categoryPage.operation}
                        </TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {categoryApps.map((app) => (
                        <TableRow key={app.id}>
                          <TableCell>{app.name}</TableCell>
                          <TableCell className="text-muted-foreground">
                            {app.path}
                          </TableCell>
                          <TableCell>
                            <Select
                              value={String(selected?.id ?? "")}
                              onValueChange={(val) => {
                                if (val) {
                                  handleAssign(app.id, Number(val))
                                } else {
                                  handleRemove(app.id)
                                }
                              }}
                            >
                              <SelectTrigger className="h-8 w-36">
                                <SelectValue />
                              </SelectTrigger>
                              <SelectContent>
                                {categoryOptions.map((opt) => (
                                  <SelectItem
                                    key={opt.categoryId}
                                    value={String(opt.categoryId)}
                                  >
                                    {opt.label}
                                  </SelectItem>
                                ))}
                              </SelectContent>
                            </Select>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>
              </TabsContent>
            </Tabs>
          </CardContent>
        </Card>
      </div>

      <Dialog open={addDialogOpen} onOpenChange={setAddDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t.categoryPage.addCategory}</DialogTitle>
          </DialogHeader>
          {addParent && (
            <p className="text-sm text-muted-foreground">
              {t.categoryPage.parentCategory}: {addParent.name}
            </p>
          )}
          <Input
            autoFocus
            placeholder={t.categoryPage.enterCategoryName}
            value={addName}
            onChange={(e) => setAddName(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") void handleAddCategory()
            }}
          />
          <DialogFooter>
            <Button variant="outline" onClick={() => setAddDialogOpen(false)}>
              {t.categoryPage.cancel}
            </Button>
            <Button onClick={() => void handleAddCategory()}>
              {t.categoryPage.confirm}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <Dialog
        open={deleteTarget !== null}
        onOpenChange={(open) => !open && setDeleteTarget(null)}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t.categoryPage.confirmDelete}</DialogTitle>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDeleteTarget(null)}>
              {t.categoryPage.cancel}
            </Button>
            <Button
              variant="destructive"
              onClick={() => {
                if (deleteTarget !== null) {
                  void handleDeleteCategory(deleteTarget)
                }
                setDeleteTarget(null)
              }}
            >
              {t.categoryPage.confirm}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}
