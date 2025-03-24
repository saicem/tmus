import { Duration, Moment } from "moment-timezone"

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

export interface FocusData {
  id: number
  start: Moment
  end: Moment
  duration: Duration
}

export interface DateGroup<T> {
  moment: Moment
  data: T[]
}

export interface AppDuration {
  app: FileDetail
  duration: Duration
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
