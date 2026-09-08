import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi'
import '@mdi/font/css/materialdesignicons.css'
import './contrast-fix.css'

export default createVuetify({
  directives,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
  theme: {
    defaultTheme: 'augustDark',
    themes: {
      augustDark: {
        dark: true,
        colors: {
          background: '#0F1117',
          surface: '#1A1D27',
          'surface-variant': '#252836',
          primary: '#FF6B35',        // August orange
          secondary: '#4ECDC4',      // Teal accent
          error: '#FF4757',          // Critical/Bug
          warning: '#FFA502',        // Major
          info: '#3742FA',           // Info
          success: '#2ED573',        // Resolved
          'on-background': '#E8E8E8',
          'on-surface': '#E8E8E8',
        },
      },
      augustLight: {
        dark: false,
        colors: {
          background: '#FAFBFC',
          surface: '#FFFFFF',
          'surface-variant': '#F1F3F5',
          primary: '#E85D26',
          secondary: '#3DB8AD',
          error: '#E8384F',
          warning: '#E89B26',
          info: '#2B35D8',
          success: '#27B864',
          'on-background': '#1A1D27',
          'on-surface': '#1A1D27',
        },
      },
    },
  },
})
