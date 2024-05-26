interface FileVersion {
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

interface FileDetail {
  id: number
  name: string
  path: string
  version?: FileVersion
}
