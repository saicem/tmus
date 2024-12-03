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
