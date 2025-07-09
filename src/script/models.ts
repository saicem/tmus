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
  id: number
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
  id: number
  focusAt: number
  blurAt: number
}

export interface IdDuration {
  appId: number
  duration: number
}

export interface DateDuration {
  date: number
  duration: number
}

export interface IdDateDuration {
  appId: number
  date: number
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
  appId: number
  dateArea: AppDurationAreaModelItem[]
  dayArea: AppDurationAreaModelItem[]
}

export interface AppDurationAreaModelItem {
  index: string
  value: number
}
