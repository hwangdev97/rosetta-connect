# 🏁 快速开始指南

这个指南将带你在 5 分钟内完成 Rosetta Connect 的安装和第一次使用。

## 📋 准备工作

### 系统要求
- **操作系统**: macOS 10.15+ / Linux / Windows 10+
- **架构**: x64 或 ARM64
- **Rust**: 1.70.0+ (仅构建时需要)
- **存储空间**: 至少 50MB

### 账号准备
- **Apple Developer 账号**: 用于访问 App Store Connect
- **OpenAI 账号** (可选): 用于 AI 翻译功能

## 🚀 第一步：安装工具

### 方法一：从源码构建 (推荐)
```bash
# 1. 克隆仓库
git clone https://github.com/yourusername/rosetta-connect
cd rosetta-connect

# 2. 构建发布版本
cargo build --release

# 3. 验证安装
./target/release/rosetta-connect --version
```

### 方法二：下载预编译版本
```bash
# 下载最新版本 (示例)
wget https://github.com/yourusername/rosetta-connect/releases/latest/download/rosetta-connect-macos
chmod +x rosetta-connect-macos
./rosetta-connect-macos --version
```

### 方法三：添加到 PATH (可选)
```bash
# 复制到系统路径
sudo cp target/release/rosetta-connect /usr/local/bin/
rosetta-connect --version  # 现在可以在任何地方使用
```

## 🔧 第二步：配置凭据

### App Store Connect API 密钥

1. **登录 [App Store Connect](https://appstoreconnect.apple.com)**

2. **生成 API 密钥**:
   - 进入 "用户和访问权限" → "密钥"
   - 点击 "生成 API 密钥" 或加号 (+)
   - 选择访问权限 (建议选择"开发者"角色)
   - 下载 `.p8` 私钥文件

3. **获取必要信息**:
   - **Issuer ID**: 在密钥页面顶部显示
   - **Key ID**: 密钥列表中的 ID 列
   - **私钥文件**: 下载的 `.p8` 文件

### OpenAI API 密钥 (可选)

1. 登录 [OpenAI Platform](https://platform.openai.com)
2. 进入 "API Keys" 页面
3. 点击 "Create new secret key"
4. 复制生成的密钥

## 📱 第三步：创建你的第一个项目

### 1. 初始化项目
```bash
# 创建项目目录
mkdir my-app-localization
cd my-app-localization

# 初始化 Rosetta Connect 项目
rosetta-connect init --bundle-id com.example.myapp
```

**输出示例:**
```
✅ Successfully initialized rosetta-connect project
📱 Bundle ID: com.example.myapp
🌍 Default locale: en-US
📝 Configuration saved to: rosetta.toml
🔑 Please edit .env file with your API credentials
```

### 2. 配置环境变量
```bash
# 编辑 .env 文件
nano .env
```

填入你的凭据:
```env
# App Store Connect API 凭据
ISSUER_ID=12345678-1234-1234-1234-123456789abc
KEY_ID=ABCD123456
PRIVATE_KEY_PATH=./AuthKey_ABCD123456.p8

# OpenAI API 密钥 (可选)
OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### 3. 放置私钥文件
```bash
# 将下载的 .p8 文件放到项目目录
cp ~/Downloads/AuthKey_ABCD123456.p8 ./
```

### 4. 检查配置
```bash
# 查看生成的配置文件
cat rosetta.toml
```

**配置文件示例:**
```toml
[app]
bundle_id = "com.example.myapp"
default_locale = "en-US"
target_locales = ["zh-Hans", "fr-FR", "de-DE"]

[assets]
default = "./screenshots/en"
zh-Hans = "./screenshots/zh"

[ai]
provider = "openai"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 1024
```

## 🌍 第四步：开始本地化工作

### 1. 拉取现有内容
```bash
rosetta-connect pull
```

**输出示例:**
```
🔄 Connecting to App Store Connect API...
📥 Downloaded app metadata for all locales
📱 Downloaded app screenshots
✅ Pull completed successfully
```

### 2. 生成 AI 翻译
```bash
# 翻译到中文和法文
rosetta-connect translate --locales zh-Hans,fr-FR
```

**输出示例:**
```
🤖 Using AI model: gpt-4o-mini
🌍 Target locales: zh-Hans, fr-FR
🔄 Translating to zh-Hans...
   ✅ App name translated
   ✅ App description translated
   ✅ Keywords translated
   ✅ What's new text translated
✅ Translation completed for 2 locales
💰 Estimated cost: $0.30
```

### 3. 预览翻译结果
```bash
# 预览中文版本
rosetta-connect preview --locale zh-Hans
```

### 4. 检查差异
```bash
rosetta-connect diff
```

**输出示例:**
```
🔍 Changes detected:
   📝 App description (zh-Hans): New translation
   📝 App description (fr-FR): New translation
   🏷️  Keywords (zh-Hans): New translation
   🏷️  Keywords (fr-FR): New translation
```

### 5. 验证内容
```bash
rosetta-connect validate
```

### 6. 推送到 App Store Connect
```bash
rosetta-connect push 2.3.4 --yes
```

**输出示例:**
```
✅ Successfully pushed version 2.3.4 to App Store Connect
📱 Updated 4 locales
🖼️  Uploaded 12 screenshots
⏱️  Total time: 8.5 seconds
```

## 🎉 恭喜！你已经完成了第一个本地化项目

### 📊 你刚才做了什么？

1. ✅ **安装了工具**: Rosetta Connect CLI
2. ✅ **配置了凭据**: App Store Connect 和 OpenAI API
3. ✅ **创建了项目**: 包含配置文件和目录结构
4. ✅ **拉取了内容**: 从 App Store Connect 获取现有数据
5. ✅ **生成了翻译**: 使用 AI 翻译到多种语言
6. ✅ **验证了内容**: 确保符合 App Store 规范
7. ✅ **推送了更新**: 上传本地化内容到 App Store Connect

### 🔄 典型的日常工作流程

```bash
# 每次发布新版本时的流程
rosetta-connect pull                    # 1. 拉取最新内容
rosetta-connect translate               # 2. 更新翻译
rosetta-connect validate                # 3. 验证内容
rosetta-connect preview --locale zh-Hans  # 4. 预览效果
rosetta-connect diff                    # 5. 检查变更
rosetta-connect push 2.3.5             # 6. 推送新版本
```

## 🔍 下一步做什么？

### 深入学习
- 📋 [命令参考](./commands.md) - 了解所有可用命令
- ⚙️ [配置详解](./configuration.md) - 自定义配置选项
- 🏗️ [技术架构](./architecture.md) - 了解工具原理

### 优化工作流程
- 💡 [最佳实践](./best-practices.md) - 提高效率的技巧
- 💰 [成本优化](./cost-optimization.md) - 降低 API 调用成本
- 🔧 [自定义模板](./custom-templates.md) - 优化 AI 翻译质量

### 故障排除
- 🆘 [常见问题](./troubleshooting.md) - 问题解决方案
- 🐛 [错误代码](./error-codes.md) - 错误信息含义
- 🔍 [调试指南](./debugging.md) - 深度排查问题

## ❓ 常见问题

### Q: 我可以不使用 AI 翻译功能吗？
A: 可以！你可以手动编辑翻译内容，只使用 `pull` 和 `push` 功能来管理 App Store Connect 的内容。

### Q: 支持哪些语言？
A: 支持 App Store Connect 支持的所有语言，包括中文简体/繁体、日语、韩语、法语、德语、西班牙语等。

### Q: AI 翻译的成本如何？
A: 成本取决于内容长度和选择的 AI 模型。使用 `rosetta-connect cost` 命令可以预估费用。

### Q: 是否支持团队协作？
A: 是的！配置文件可以共享，每个成员配置自己的 `.env` 文件即可。详见 [团队协作](./team-collaboration.md)。

## 📞 获取帮助

- 🐛 [提交问题](https://github.com/yourusername/rosetta-connect/issues)
- 💬 [讨论区](https://github.com/yourusername/rosetta-connect/discussions)
- 📚 [完整文档](./README.md)

---

<p align="center">
  🎯 现在你已经准备好使用 Rosetta Connect 来加速你的应用本地化了！
</p>