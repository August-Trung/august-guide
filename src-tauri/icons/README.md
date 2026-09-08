# Icons

Tauri requires icon files in this directory to build.

## For Development (T1.01)

Run the following command to generate placeholder icons from the SVG:

```bash
# Requires ImageMagick or use Tauri's built-in icon generator:
npm run tauri icon public/favicon.svg
```

This will auto-generate all required icon sizes:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

## Manual Workaround

If the above command fails, you can download a placeholder PNG and run:

```bash
npm run tauri icon path/to/your/icon.png
```

The icon should be at least 1024x1024 pixels, square, PNG format.

## Production

Replace with the final August Mark branded icon before v0.1 release.
See: https://v2.tauri.app/distribute/app-icon/
