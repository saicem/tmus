import { reactive } from "vue"

export const languageStore = reactive({
  language: "zh-CN",
  setLanguage(language: string) {
    this.language = language
  },
})

export const tabStore = reactive({
  cur: "home",
  change(next: string) {
    this.cur = next
  },
})
