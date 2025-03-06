import { Duration } from "moment-timezone"

export const formatDuration = (duration: Duration) => {
  let parts = []
  if (duration.days() > 0) {
    parts.push(duration.days() + "d")
  }
  if (duration.hours() > 0) {
    parts.push(duration.hours() + "h")
  }
  if (duration.minutes() > 0) {
    parts.push(duration.minutes() + "m")
  }
  if (duration.seconds() > 0) {
    parts.push(duration.seconds() + "s")
  }
  return parts.join(" ")
}
