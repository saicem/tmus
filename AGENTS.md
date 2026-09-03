# Tmus 项目指南

## 项目介绍

Tmus 是一款 Windows 桌面应用（仓库：`saicem/tmus`），在后台自动记录各应用的前台使用时长，并提供首页总览、统计图表、时间线、应用详情、分类管理和命令行查询。核心统计引擎用 Rust 编写，资源占用低；界面完全本地化，无账号体系。

技术栈：

- 前端：Vite + TypeScript + pnpm，React 19，shadcn/ui + Tailwind CSS v4，Recharts，zustand，react-router，date-fns，lucide-react
- 后端：Tauri 2 + Rust（`src-tauri`），统计引擎独立 crate（`tmus-engine`）
- 打包/分发：NSIS 安装器，GitHub Actions 自动出 release，内置 tauri-plugin-updater 更新

主要页面（`src/pages/`）：`Overview`（首页）、`Statistic`（统计）、`Timeline`（时间线）、`Application`（应用列表）、`Detail`（应用详情）、`Category`（分类）、`Setting`（设置）。

内置 CLI：`tmus time`、`tmus usage`、`tmus app ...`、`tmus records`、`tmus category ...`，均支持 `--json` 输出；`--nw` 为无窗口守护模式。

## 目录结构

```
tmus/
├── src/                    # 前端
│   ├── components/         # ui/ shadcn 组件；layout/ 布局；common/ 通用；statistic/ 统计组件
│   ├── pages/              # 页面
│   ├── lib/                # cmd.ts 命令封装、models.ts 类型、i18n.ts、state.ts、time-util.ts
│   ├── index.css           # Tailwind v4 入口与品牌主题
│   └── main.tsx
├── src-tauri/              # Rust 后端（Tauri 应用）
│   ├── src/app/            # 托盘、窗口、定时器、更新、常量（APP_NAME/APP_TITLE/数据目录）
│   ├── src/cmd/            # Tauri 命令（statistic/duration/category/app_detail 等）
│   ├── src/cli/            # CLI 子命令
│   ├── src/state/          # config/category/rule/i18n 状态
│   ├── src/util/
│   └── windows/hooks.nsi   # NSIS 安装/卸载钩子（维护 PATH）
├── tmus-engine/            # 追踪引擎：前台窗口监控、二进制记录与按日索引、应用去重映射
├── spec/                   # 规范文档
├── doc/                    # 截图等
└── .github/workflows/      # 推送到 release 分支时自动发布
```

## 快速开始

```bash
pnpm install
pnpm tauri dev      # 本地开发（仅 Windows；标题/托盘显示 "Tmus Dev"）
pnpm run build      # 前端编译检查（tsc -b && vite build）
cargo check         # 后端检查（仓库根目录，workspace 含 src-tauri 与 tmus-engine）
pnpm tauri build    # 打包
```

提交时 husky + lint-staged 会自动执行 eslint/prettier；也可手动 `pnpm lint`、`pnpm format`。

## 通用规范

- 代码内注释使用英文，其余（文档、UI 说明等）使用中文。
- 前端硬编码文本须配置多语言：前端文案在 `src/lib/i18n.ts`（通过 `useI18n()` 读取），后端托盘等文案在 `src-tauri/src/state/i18n.rs`。
- 可复用组件提取到 `src/components` 下；单页面过于复杂时拆分成多个组件。
- 后端修改后运行 `cargo check`；涉及 Rust JSON 数据处理时参考 `spec/tool/serde.md`。
- 命名参考 `spec/naming/tree.md`；接口契约详见 `spec/communicate.md`。

## 架构要点（接手前必读）

1. **前后端通信**：前端统一经 `src/lib/cmd.ts` 的 `ivk`（`invoke` 封装）调用 Tauri 命令，类型定义在 `src/lib/models.ts`。命令接收**单个 request 对象**（不用散装参数），字段 camelCase，时间戳统一为毫秒。命令返回 `Result<T, TmusError>`，错误含机器可读 `code` 与人类可读 `message`，前端按 `code` 映射文案。
2. **时间模型（极易踩坑）**：存储层只认 UTC。`FocusRecord` 含 `focus_at`/`blur_at`（UTC 毫秒）；二进制记录拆分为不跨 UTC 日、单条 ≤0.76 天的片段，便于按日索引。所有"按本地日/时聚合"只在查询时用 `timezoneOffset` 换算：`offset = UTC − local`（毫秒，如上海 = −8h，即 `new Date().getTimezoneOffset() * 60000`）。配置 `timezoneOffset` 为 `null` 表示自动推断；页面统一传 `useAppStore.getState().config.timezoneOffset`。**不要修改已存储的时间戳**。
3. **引擎与数据**：`tmus-engine` 负责前台窗口轮询（`tracker/`）、记录落盘与索引（`storage/focus_record.rs`、`focus_index.rs`）、应用识别与去重（`storage/focus_app.rs`）。数据目录：`%APPDATA%/tmus`（debug 构建为 `tmus.dev`），含 `state.json`、`rule.json`、`category.json` 等。GUI/CLI 入口都先 `engine_start(data_dir, EngineMode::Working, rule_filter)`。
4. **应用识别 / WindowsApps**：同一应用可能同时出现在 `C:\Program Files\WindowsApps\...` 与 Program Files 等不同路径，去重/合并依赖 canonical id map（`focus_app::get_canonical_id_map`），不要假设一条路径就是一个应用。图标优先读取 exe 内嵌图标；WindowsApps 的磁贴资产不可靠，不要依赖。
5. **窗口与托盘**：`decorations: false` 无边框是刻意设计。Windows 11 开启阴影时，系统非客户区会让内容离边缘内缩约 2~8px，属 tao 行为；尝试 `shadow: false` 会失去阴影与圆角（体验更差），此前已回退，不要再改。托盘悬停文案为 `APP_TITLE`（`cfg(debug_assertions)` 时为 "Tmus Dev"，release 为 "Tmus"）。
6. **CLI 控制台**：release 使用 GUI subsystem（`windows_subsystem = "windows"`），只有解析到真实 CLI 子命令时才 `AttachConsole`（`--nw`/无参启动也带参数，不要用 `args().count() > 1` 判断）。`--help/--version` 在 clap 解析失败分支处理。
7. **安装器**：NSIS 钩子（`src-tauri/windows/hooks.nsi`）负责安装时把 exe 目录加入用户 PATH、卸载时移除，保证命令行可用。

## 提交与发布

- 提交代码使用 commit-code skill（约定式提交、英文消息、按功能拆分多个 commit）。
- 版本号需同步两处：`src-tauri/Cargo.toml` 与 `src-tauri/tauri.conf.json`（语义化版本；"小版本"指第三位 patch）。
- 发布流程：提交到 `main` → 推送到 `release` 分支 → GitHub Actions（`.github/workflows/publish-to-auto-release.yml`）自动产出 `vX.Y.Z` draft release（含 updater 的 `latest.json`，需要 `TAURI_SIGNING_PRIVATE_KEY` secret）→ 用 `gh` 编辑 release notes。
- 本机推送 GitHub 时 SSH 22 端口可能被拦，可用 HTTPS：`git push https://github.com/saicem/tmus.git <branch>`。

## 注意事项

- 不要自行编写/启动后台执行、定时、watch、daemon 类脚本；需要时把命令告诉用户，由用户手动启动。
- 改动前端后运行 `pnpm run build`，改动后端后运行 `cargo check`（两者都涉及则都跑）。
- 时间线/统计等页面存在不少"看似修好实则偏移"的历史坑（如按天数组的 0/1 基索引、跨天分桶），改动日期相关逻辑时先在真实数据上核对具体日期。
- 详细规范索引：`spec/spec.md`、`spec/communicate.md`、`spec/i18n.md`、`spec/feature/*.md`。
