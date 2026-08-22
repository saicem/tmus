# 前后端交互规范

## 前端调用后端命令

- 前端调用后端命令统一封装在 `src/lib/cmd.ts`
- 数据结构定义在 `src/lib/models.ts`
- 所有调用通过 `ivk`（`invoke` 的封装）进行，命令名与参数结构一一对应

## 后端命令定义

- 后端命令在 `src-tauri/src/cmd` 目录下定义
- 使用 `#[tauri::command]` 宏标记命令函数，统一注册在 `src-tauri/src/cmd/mod.rs` 的 `handler()` 中
- 命令参数与返回类型使用 serde 序列化，`#[serde(rename_all = "camelCase")]` 将字段名转为驼峰命名

## 接口契约约定

- **参数风格**：命令统一接收单个 request 对象（如 `DurationStatisticRequest`、`AppDurationAreaRequest`），禁止散装参数
- **错误处理**：命令返回 `Result<T, TmusError>`。`TmusError` 包含 `code`（机器可读错误码）与 `message`（人类可读信息），前端按 `code` 映射本地化文案
- **命名**：命令名使用 snake_case（如 `get_app_config`），字段名使用 camelCase（如 `startTimestamp`）
- **时间戳**：统一使用毫秒时间戳
- **轻量化**：统计类接口只返回展示所需字段（如 `AppSimple { id, name }`），不返回图标等重字段；原始记录查询走 CLI
- **职责**：调试/内部查询接口（原始记录、索引等）不暴露为 Tauri 命令，由 CLI 提供
