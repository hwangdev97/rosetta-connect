# Rosetta Connect

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/OpenAI-412991?style=for-the-badge&logo=openai&logoColor=white" alt="OpenAI">
  <img src="https://img.shields.io/badge/App_Store_Connect-0D96F6?style=for-the-badge&logo=apple&logoColor=white" alt="App Store Connect">
</p>

<p align="center">
  <strong>一个强大的 CLI 工具，用于自动化 App Store Connect 本地化工作流程</strong>
</p>

<p align="center">
  使用 AI 驱动的翻译技术，让你的 iOS 应用轻松实现全球化发布
</p>

---

## ✨ 核心特性

| 特性 | 说明 |
|------|------|
| 🚀 **极速体验** | 基于 Rust 构建，启动快速，性能卓越 |
| 🤖 **AI 翻译** | 集成 OpenAI GPT，智能生成高质量本地化内容 |
| 📱 **完整工作流** | 一键完成拉取→翻译→审查→推送的完整流程 |
| 🖼️ **截图管理** | 自动化截图上传，支持多设备多语言 |
| 💰 **成本透明** | 实时估算 API 成本，避免意外费用 |
| 🔧 **开箱即用** | 零配置启动，内嵌运行时，无外部依赖 |

## 🚀 快速开始

### 安装
```bash
git clone https://github.com/yourusername/rosetta-connect
cd rosetta-connect
cargo build --release
```

### 30 秒体验
```bash
# 1. 初始化项目
./target/release/rosetta-connect init --bundle-id com.example.myapp

# 2. 开始本地化工作流
./target/release/rosetta-connect pull
./target/release/rosetta-connect translate --locales zh-Hans,fr-FR
./target/release/rosetta-connect push 2.3.4 --yes
```

**🎉 就是这么简单！你的应用已经支持多语言了。**

## 📚 完整文档

### 📖 [Help Center →](./help-center/README.md)

| 文档类型 | 链接 | 说明 |
|----------|------|------|
| 🏁 **新手入门** | [入门指南](./help-center/getting-started.md) | 从零开始，详细的安装和配置教程 |
| 📋 **命令参考** | [命令手册](./help-center/commands.md) | 所有命令的详细说明和示例 |
| 🏗️ **技术原理** | [架构文档](./help-center/architecture.md) | 深入了解工具的技术实现 |
| ⚙️ **配置指南** | [配置文档](./help-center/configuration.md) | 详细的配置选项说明 |
| 🔧 **故障排除** | [FAQ](./help-center/troubleshooting.md) | 常见问题和解决方案 |
| 💡 **最佳实践** | [使用技巧](./help-center/best-practices.md) | 高效使用的技巧和建议 |

## ⚡ 核心命令概览

```bash
# 项目管理
rosetta-connect init --bundle-id <your-bundle-id>  # 初始化项目
rosetta-connect pull                                # 拉取远程内容
rosetta-connect push <version>                      # 推送到 App Store

# AI 翻译
rosetta-connect translate                           # 翻译所有语言
rosetta-connect translate --locales zh-Hans,fr-FR  # 翻译指定语言
rosetta-connect cost --detailed                     # 查看成本估算

# 内容管理
rosetta-connect diff                                # 对比差异
rosetta-connect preview --locale zh-Hans           # 预览内容
rosetta-connect validate                            # 验证合规性
```

👆 **更多命令详情请查看** → [📋 完整命令手册](./help-center/commands.md)

## 🔥 项目状态

| 模块 | 状态 | 说明 |
|------|------|------|
| 🏗️ **CLI 框架** | ✅ 完成 | 完整的命令行界面和参数解析 |
| ⚙️ **配置系统** | ✅ 完成 | 支持 `rosetta.toml` 和环境变量 |
| 🎨 **用户界面** | ✅ 完成 | 漂亮的进度条和状态显示 |
| 🔌 **Node.js 桥接** | 🚧 开发中 | 当前使用模拟 API 响应 |
| 🤖 **AI 翻译集成** | 📋 计划中 | 等待 OpenAI API 集成 |
| 📱 **App Store API** | 📋 计划中 | 等待真实 API 集成 |

> **当前版本**: MVP 演示版本，使用模拟数据展示完整功能流程

## 🤝 贡献和支持

- 🐛 **发现问题**: [提交 Issue](https://github.com/yourusername/rosetta-connect/issues)
- 💡 **功能建议**: [讨论区](https://github.com/yourusername/rosetta-connect/discussions)  
- 🔧 **参与开发**: [贡献指南](./help-center/contributing.md)
- 📖 **详细文档**: [Help Center](./help-center/README.md)

## 📄 许可证

MIT License - 详见 [LICENSE](./LICENSE) 文件

---

<p align="center">
  <strong>让 iOS 应用的全球化变得简单高效 🌍</strong>
</p>

<p align="center">
  Made with ❤️ by the Rosetta Connect team
</p>