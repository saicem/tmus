export type AppId = number
export type CategoryId = number

export interface FileVersion {
  comments?: string
  internalName?: string
  productName?: string
  companyName?: string
  legalCopyright?: string
  productVersion?: string
  fileDescription?: string
  legalTrademarks?: string
  privateBuild?: string
  fileVersion?: string
  originalFilename?: string
  specialBuild?: string
}

export interface FileDetail {
  id: AppId
  name: string
  path: string
  exist: boolean
  /**
   * Base64 png
   */
  icon: string
  version?: FileVersion
}

export interface FocusRecord {
  id: AppId
  focusAt: number
  blurAt: number
}

export interface IdDuration {
  appId: AppId
  duration: number
}

export interface DurationStat {
  appId: AppId | null
  intervalStart: number
  duration: number
}

export interface DateGroup<T> {
  moment: Date
  data: T[]
}

export interface AppDuration {
  app: FileDetail
  duration: number
}

export interface RuleConfig {
  exclude: ExcludeRuleItem[]
  include: IncludeRuleItem[]
  merge: MergeRuleItem[]
}

export interface ExcludeRuleItem {
  path: string
}

export interface IncludeRuleItem {
  path: string
}

export interface MergeRuleItem {
  path: string
  toPath: string
}

export interface TagConfig {
  appPath: string
  tags: string[]
}

export interface AppMeta {
  initialTimestamp: number
  tmusVersion: string
}

export interface FileIndexRecord {
  date_time: string
  start_index: number
}

export interface UpdateMetadata {
  version: string
  currentVersion: string
}

export interface DownloadEvent {
  event: "Started" | "Progress" | "Finished"
  data: {
    contentLength?: number
    chunkLength?: number
  }
}

export interface AppDurationAreaModel {
  appId: AppId
  dateArea: AppDurationAreaModelItem[]
  dayArea: AppDurationAreaModelItem[]
}

export interface AppDurationAreaModelItem {
  index: string
  value: number
}

export interface McpServerStatus {
  running: boolean
  port?: number
}

export interface Category {
  id: CategoryId
  name: string
  children: Category[]
  appIds: AppId[]
}

export interface AddCategoryRequest {
  name: string
  parentId: CategoryId
}

export interface UpdateCategoryRequest {
  id: CategoryId
  name: string
}

export interface DeleteCategoryRequest {
  id: CategoryId
}

export interface AssignCategoryRequest {
  appId: AppId
  categoryId: CategoryId
}

export interface RemoveAppCategoryRequest {
  appId: AppId
}

export enum TimeSpan {
  Day = "Day",
  Week = "Week"
}

export interface AppStatisticDetail {
  value: number
  app: FileDetail
}

export interface CategoryStatisticDetail {
  value: number
  category: CategorySimple
}

export interface CategorySimple {
  id: CategoryId
  parentId: CategoryId
  name: string
}

export interface AppDurationRequest {
  startTime: number
  endTime: number
  categoryId?: CategoryId
}

export interface AppDurationResponse {
  detail: AppStatisticDetail[]
}

export interface AppDayCountRequest {
  startTime: number
  endTime: number
  categoryId?: CategoryId
}

export interface AppDayCountResponse {
  detail: AppStatisticDetail[]
}

export interface CategoryDurationRequest {
  startTime: number
  endTime: number
  categoryIds: CategoryId[]
}

export interface CategoryDurationResponse {
  detail: CategoryStatisticDetail[]
}

export interface CategoryDayCountRequest {
  startTime: number
  endTime: number
  categoryIds: CategoryId[]
}

export interface CategoryDayCountResponse {
  detail: CategoryStatisticDetail[]
}

export interface RhythmGroup {
  startTime: number
  endTime: number
  categoryId: CategoryId
}

export interface RhythmRequest {
  groups: RhythmGroup[]
  span: TimeSpan
  granularity: number
}

export interface RhythmDataResponse {
  values: number[][]
}

