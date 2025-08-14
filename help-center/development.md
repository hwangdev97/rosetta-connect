# 🧑‍💻 开发环境与本地调试

本指南帮助你在本地搭建开发环境、构建二进制、进行基础调试。

## ✨ 前置要求
- 操作系统：macOS / Linux / Windows 10+
- Rust：1.70+
- 可选：`cargo-watch`、`just`

## 🚀 克隆与构建
```bash
git clone https://github.com/yourusername/rosetta-connect
cd rosetta-connect
cargo build --release
./target/release/rosetta-connect --version
```

## 🧪 快速冒烟测试
```bash
bash ./快速测试脚本.sh
```
更多场景请参见仓库根目录的 `测试指南.md`。

## 🗂 代码结构
```text
crates/
  rc-cli/   # CLI 与命令解析
  rc-node/  # 与 JS/TS 的桥接层（后续完善）
js/
  asc.ts    # App Store Connect 相关脚本（后续完善）
help-center/ # 文档
```

## 🔧 本地调试
```bash
# 详细输出
./target/release/rosetta-connect --verbose pull

# 转发 JS 日志（若实现）
ROSETTA_DEBUG_JS=1 ./target/release/rosetta-connect pull
```

## ✅ 代码质量
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## 📎 相关文档
- [快速开始](./getting-started.md)
- [命令参考](./commands.md)
- [配置详解](./configuration.md)
- [最佳实践](./best-practices.md)
- [贡献指南](./contributing.md)