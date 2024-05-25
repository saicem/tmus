type FileVersion = {
    comments?: string,
    internalName?: string,
    productName?: string,
    companyName?: string,
    legalCopyright?: string,
    productVersion?: string,
    fileDescription?: string,
    legalTrademarks?: string,
    privateBuild?: string,
    fileVersion?: string,
    originalFilename?: string,
    specialBuild?: string,
}

type FileDetail = {
    id: number,
    path: string,
    version?: FileVersion
}