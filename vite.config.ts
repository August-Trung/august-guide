import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vuetify from 'vite-plugin-vuetify'
import { fileURLToPath, URL } from 'node:url'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    vuetify({ autoImport: true })
  ],

  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },

  // Prevent Vite from obscuring Rust errors.
  clearScreen: false,

  server: {
    // Tauri expects a fixed port; fail if it's not available.
    port: 1420,
    strictPort: true,
    watch: {
      // Tell Vite to ignore watching `src-tauri`.
      ignored: ['**/src-tauri/**'],
    },
  },

  // Build config for production.
  build: {
    // Tauri targets Chromium on Windows/Linux, WebKit on macOS.
    target: process.env.TAURI_ENV_PLATFORM === 'windows'
      ? 'chrome105'
      : process.env.TAURI_ENV_PLATFORM === 'macos'
        ? 'safari14'
        : 'chrome105',
    // Don't minify for debug builds.
    minify: process.env.TAURI_ENV_DEBUG ? false : 'esbuild',
    // Produce sourcemaps for debug builds.
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    rollupOptions: {
      input: {
        main: fileURLToPath(new URL('./index.html', import.meta.url)),
        overlay: fileURLToPath(new URL('./overlay.html', import.meta.url))
      }
    }
  },
}))
