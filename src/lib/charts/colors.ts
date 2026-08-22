/**
 * Chart palette sourced from the app theme CSS variables so charts follow
 * the active light/dark theme automatically.
 */
export function getChartColors(): string[] {
  if (typeof document === "undefined") {
    return ["#396afc", "#668bf9", "#92acf6", "#c792ea", "#26c6da"]
  }
  const css = getComputedStyle(document.documentElement)
  return ["--chart-1", "--chart-2", "--chart-3", "--chart-4", "--chart-5"]
    .map((v) => css.getPropertyValue(v).trim())
    .filter(Boolean)
}

export function getMutedColor(): string {
  if (typeof document === "undefined") return "#808080"
  return (
    getComputedStyle(document.documentElement)
      .getPropertyValue("--muted-foreground")
      .trim() || "#808080"
  )
}

export function getGridColor(): string {
  if (typeof document === "undefined") return "#e5e5e5"
  return (
    getComputedStyle(document.documentElement)
      .getPropertyValue("--border")
      .trim() || "#e5e5e5"
  )
}
