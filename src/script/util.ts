export function groupBy<T, K, V>(
  arr: T[],
  fn: (item: T) => [K, V]
): Map<K, V[]> {
  return arr.reduce((map, item) => {
    const [k, v] = fn(item)
    if (!map.has(k)) {
      map.set(k, [v])
    } else {
      map.get(k)?.push(v)
    }
    return map
  }, new Map<K, V[]>())
}

export function toMap<T, K, V>(arr: T[], fn: (item: T) => [K, V]): Map<K, V> {
  return arr.reduce((map, item) => {
    const [k, v] = fn(item)
    map.set(k, v)
    return map
  }, new Map<K, V>())
}

export function range(start: number, end: number, step: number = 1) {
  if (step === 0) {
    throw new Error("Step cannot be zero to avoid infinite loop.")
  }
  const result = []
  if (step > 0) {
    for (let i = start; i < end; i += step) {
      result.push(i)
    }
  } else {
    for (let i = start; i > end; i += step) {
      result.push(i)
    }
  }
  return result
}
