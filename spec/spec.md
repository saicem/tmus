# 项目概述

这是一个基于 Tauri(Rust + Vue3) 的 Windows 桌面应用，用于管理应用的使用统计和分类。
应用使用情况的记录和查询由 tmus-engine 实现。

## 技术栈

- Vite
- pnpm
- Tauri
- Rust
- Vue3
- TypeScript
- Element Plus

## 项目结构

只包含重要的文件和文件夹

```
tmus/
├── src/                  # 前端源代码
│   ├── components/       # 前端组件
│   ├── pages/            # 页面组件（包括 Category.vue）
│   ├── script/           # 脚本文件（包括 cmd.ts、models.ts）
│   ├── App.vue           # 应用主组件
│   └── main.ts           # 应用入口
├── src-tauri/            # Tauri 后端代码
│   ├── src/              # Rust 源代码
│   │   ├── app/          # 应用相关功能
│   │   ├── cmd/          # Tauri 命令
│   │   ├── mcp/          # MCP 服务
│   │   └── main.rs       # 后端入口
├── tmus-engine/          # 应用使用统计引擎
├── spec/                 # 项目规范文件
├── AGENTS.md             # Agent 相关文档
├── package.json          # 前端依赖配置
├── Cargo.toml            # 项目依赖配置
└── vite.config.ts        # Vite 配置
```

## 规范文档索引

- [前后端交互规范](communicate.md)
- [国际化（多语言，多时区）](i18n.md)
- [功能规范](feature/feature.md)
- [分类功能](feature/category.md)
- [树命名规范](naming/tree.md)
- [Rust JSON 处理](tool/serde.md)
