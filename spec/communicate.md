# 前后端交互规范

## 前端调用后端命令

- 前端调用后端函数应统一写在 `src/script/cmd.ts` 文件中
- 使用的数据结构定义在 `src/script/models.ts` 文件中

## 后端命令定义

- 后端命令在 `src-tauri/src/cmd` 目录下定义
- 使用 `#[tauri::command]` 宏标记命令函数
- 命令参数和返回值的类型定义在 `src-tauri/src/cmd` 目录下，使用 serde 进行序列化/反序列化，使用 `#[serde(rename_all = "camelCase")]` 注解将字段名转换为驼峰命名法
