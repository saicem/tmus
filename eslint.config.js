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
      "@typescript-eslint/naming-convention": [
        "error",
        {
          selector: "function",
          format: ["camelCase"],
        },
      ],
      "import/order": [
        "error",
        {
          groups: [
            "builtin", // Built-in imports (e.g. 'path')
            "external", // External packages (e.g. 'react', '@org/package')
            "internal", // Internal modules (e.g. '@/components')
            "parent", // Parent directories (e.g. '../')
            "sibling", // Same folder (e.g. './OtherComponent')
            "index", // index files (e.g. './')
          ],
          pathGroups: [
            {
              pattern: "react",
              group: "builtin",
              position: "before",
            },
            {
              pattern: "@/!(..)/**", // e.g., '@/store', '@/utils'
              group: "internal",
              position: "after",
            },
          ],
          pathGroupsExcludedImportTypes: ["react"],
          "newlines-between": "always",
          alphabetize: {
            order: "asc",
            caseInsensitive: true,
          },
        },
      ],
    },
  },
  eslint.configs.recommended,
  ...tseslint.configs.strict,
  ...tseslint.configs.stylistic
)
