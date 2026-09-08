import { listen, Event } from '@tauri-apps/api/event'

/**
 * Register a listener for a Tauri event.
 * Returns a promise that resolves to the unlisten function.
 */
export async function listenToEvent<T>(
  eventName: string,
  handler: (payload: T) => void
): Promise<() => void> {
  const unlisten = await listen<T>(eventName, (event: Event<T>) => {
    handler(event.payload)
  })
  return unlisten
}
