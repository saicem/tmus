import {reactive} from "vue"

export const themeStore = reactive({
    theme: "light",
    isLight() {
        return this.theme === "light"
    },
    isDark() {
        return this.theme === "dark"
    },
    setTheme(theme: "light" | "dark") {
        this.theme = theme
    },
})

export const languageStore = reactive({
    language: "zh-CN",
    setLanguage(language: string) {
        this.language = language
    }
})

export const tabStore = reactive({
    cur: "home",
    change(next: string) {
        this.cur = next
    }
})
