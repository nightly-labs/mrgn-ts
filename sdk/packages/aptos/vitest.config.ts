import tsconfigPath from 'vite-tsconfig-paths'
import { defineConfig } from 'vitest/config'

export default defineConfig({
  plugins: [tsconfigPath()],
  test: {
    testTimeout: 10000
    // singleThread: true
  }
})
