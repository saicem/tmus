# Rust JSON 处理

使用 serde 库处理 JSON 数据。

## 数据结构标记

所有的数据结构都要标记为：

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
```

## 工具函数

- 当需要直接将数据序列化到文件时，使用 `crate::util::save_json` 函数
- 当需要将文件反序列化到数据时，使用 `crate::util::load_json` 函数
