export interface AacpExportPayload {
  issueId: string
  workspacePath: string
  suspectedFiles: string[]
  outputDir: string
  compressZip: boolean
}
