declare module 'fix-webm-duration' {
  export default function fixWebmDuration(
    blob: Blob,
    duration: number,
    options?: { logger?: false | ((msg: string) => void) }
  ): Promise<Blob>;
}
