import { FileDetail } from "@/script/data.ts"
import * as localforage from "localforage"

export const saveApp = async (app: FileDetail) => {
  await localforage.setItem(app.id.toString(), app)
}
export const getApp = async (id: number) => {
  return await localforage.getItem<FileDetail>(id.toString())
}
