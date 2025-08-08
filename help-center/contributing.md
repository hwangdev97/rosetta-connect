# 🤝 贡献指南

感谢你对 Rosetta Connect 的关注！欢迎通过 Issue、PR 参与改进。

## 🧭 工作流程
- 提交 Issue 描述问题/需求（包含环境、复现步骤）
- Fork 仓库并创建分支
- 在分支中开发并补充文档
- 通过 Pull Request 提交，关联 Issue，等待评审

## 🌿 分支与提交
- 分支命名：`feat/xxx`、`fix/xxx`、`docs/xxx`、`chore/xxx`
- 提交规范：推荐使用 Conventional Commits
  - `feat: 添加 translate 的 --dry-run 选项`
  - `fix: 修复 preview 在空 locale 下崩溃`
  - `docs: 完善安装与快速开始`

## 🧑‍💻 开发环境
- 需要 Rust 1.70+
- 构建与自测：
  ```bash
  cargo build --release
  bash ./快速测试脚本.sh
  ```
- 质量检查：
  ```bash
  cargo fmt --all
  cargo clippy --all-targets -- -D warnings
  ```

## 🧪 测试与验证
- 手动验证：参考仓库根目录的 `测试指南.md`
- 快速冒烟：运行 `快速测试脚本.sh`

## 📄 文档约定
- 所有用户向文档在 `help-center/`
- 新增命令或参数时：
  - 更新 `help-center/commands.md`
  - 如影响配置，更新 `help-center/configuration.md`
  - 如影响流程，更新 `help-center/getting-started.md`

## 🔐 安全
- 请勿在 Issue/PR 中提交私密凭据（如 `.p8`、API Key）
- 使用 `.env` 管理本地敏感信息

感谢你的贡献！