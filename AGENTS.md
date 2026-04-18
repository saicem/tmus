# 通用规范

代码内注释需要使用英文，其余使用中文。

# 前端规范

前端代码检查 运行 `pnpm run build` 检查代码是否能够正常编译
前端的硬编码文本需要多语言支持配置在 `src/script/i18n.ts` 文件中。
可复用的组件需要提取到 `src/components` 目录下。
如果单页面过于复杂也需要将页面拆分成多个组件。

# 后端规范

修改后端代码后检查，运行 `cargo check` 检查代码是否符合 Rust 规范
当涉及 rust JSON 数据处理时参考 [serde](spec/tool/serde.md)。

# 命名规范

- [树命名规范](spec/naming/tree.md)