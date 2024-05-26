// @ts-check

import eslint from "@eslint/js"
import tseslint from "typescript-eslint"

export default tseslint.config(
  {
    rules: {
      quotes: "off",
      "@typescript-eslint/quotes": ["error", "double"],
      "no-extra-semi": "off",
      "@typescript-eslint/no-extra-semi": "error",
    },
  },
  eslint.configs.recommended,
  ...tseslint.configs.strict,
  ...tseslint.configs.stylistic
)
