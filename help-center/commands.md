# 📋 命令参考手册

Rosetta Connect 提供了一套完整的命令来管理应用的本地化工作流程。本文档详细介绍每个命令的用法、参数和示例。

## 🔍 命令速查表

| 命令 | 用途 | 常用参数 |
|------|------|----------|
| `init` | 初始化新项目 | `--bundle-id`, `--default-locale` |
| `pull` | 拉取远程内容 | 无 |
| `push` | 推送到远程 | `<version>`, `--yes` |
| `translate` | AI 翻译 | `--locales`, `--model` |
| `diff` | 对比差异 | 无 |
| `preview` | 预览内容 | `--locale` |
| `validate` | 验证内容 | 无 |
| `cost` | 成本估算 | `--detailed` |
| `rollback` | 版本回滚 | `<version>` |
| `template` | 模板管理 | `list`, `create`, `edit`, `delete` |

## 📖 详细命令说明

### `rosetta-connect init`
**用途**: 初始化新的本地化项目

#### 语法
```bash
rosetta-connect init [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `--bundle-id <BUNDLE_ID>` | String | 是 | 应用的 Bundle ID | 无 |
| `--default-locale <LOCALE>` | String | 否 | 默认语言区域 | `en-US` |
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 基本初始化
rosetta-connect init --bundle-id com.example.myapp

# 指定默认语言为中文
rosetta-connect init --bundle-id com.example.myapp --default-locale zh-Hans

# 使用自定义配置文件
rosetta-connect init --bundle-id com.example.myapp --config my-config.toml
```

#### 输出文件
- `rosetta.toml` - 项目配置文件
- `.env` - 环境变量模板
- `screenshots/en/` - 英文截图目录
- `screenshots/zh/` - 中文截图目录

---

### `rosetta-connect pull`
**用途**: 从 App Store Connect 拉取当前的本地化内容

#### 语法
```bash
rosetta-connect pull [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 基本拉取
rosetta-connect pull

# 详细模式
rosetta-connect pull --verbose
```

#### 功能说明
1. 连接到 App Store Connect API
2. 下载应用元数据 (名称、描述、关键词等)
3. 下载应用截图
4. 保存到本地文件系统

---

### `rosetta-connect push`
**用途**: 推送本地化内容到 App Store Connect

#### 语法
```bash
rosetta-connect push <VERSION> [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `<VERSION>` | String | 是 | 版本号 | 无 |
| `--yes` | Flag | 否 | 跳过确认提示 | false |
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 交互式推送
rosetta-connect push 2.3.4

# 自动确认推送
rosetta-connect push 2.3.4 --yes

# 推送特定配置
rosetta-connect push 2.3.4 --config production.toml
```

#### 功能说明
1. 验证本地内容
2. 上传元数据到 App Store Connect
3. 上传截图
4. 显示进度和统计信息

---

### `rosetta-connect translate`
**用途**: 使用 AI 生成本地化翻译

#### 语法
```bash
rosetta-connect translate [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `--locales <LOCALES>` | String[] | 否 | 目标语言(逗号分隔) | 配置文件中的 `target_locales` |
| `--model <MODEL>` | String | 否 | AI 模型 | 配置文件中的 `model` |
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 翻译所有配置的语言
rosetta-connect translate

# 翻译特定语言
rosetta-connect translate --locales zh-Hans,fr-FR,de-DE

# 使用特定模型
rosetta-connect translate --model gpt-4

# 组合使用
rosetta-connect translate --locales zh-Hans --model gpt-4 --verbose
```

#### 支持的语言代码
| 语言 | 代码 | 语言 | 代码 |
|------|------|------|------|
| 中文简体 | `zh-Hans` | 英语 | `en-US` |
| 中文繁体 | `zh-Hant` | 日语 | `ja` |
| 法语 | `fr-FR` | 韩语 | `ko` |
| 德语 | `de-DE` | 西班牙语 | `es-ES` |
| 意大利语 | `it` | 葡萄牙语 | `pt-BR` |
| 俄语 | `ru` | 阿拉伯语 | `ar` |

#### 支持的 AI 模型
| 模型 | 速度 | 质量 | 成本 | 建议用途 |
|------|------|------|------|----------|
| `gpt-4o-mini` | 快 | 高 | 低 | 日常翻译 |
| `gpt-4o` | 中 | 很高 | 中 | 重要发布 |
| `gpt-4-turbo` | 中 | 很高 | 高 | 专业翻译 |
| `gpt-3.5-turbo` | 很快 | 中 | 很低 | 快速原型 |

---

### `rosetta-connect diff`
**用途**: 显示本地内容与远程内容的差异

#### 语法
```bash
rosetta-connect diff [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 查看所有差异
rosetta-connect diff

# 详细差异信息
rosetta-connect diff --verbose
```

#### 输出格式
```
🔍 Changes detected:
   📝 App description (en-US): Modified
   🖼️  Screenshots (zh-Hans): 2 added, 1 removed
   🏷️  Keywords (fr-FR): Updated
   ✨ What's new (de-DE): New content

📈 Summary:
   • 4 locales with changes
   • 7 total modifications
   • Ready for push
```

---

### `rosetta-connect preview`
**用途**: 本地预览本地化内容

#### 语法
```bash
rosetta-connect preview [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `--locale <LOCALE>` | String | 否 | 预览的语言 | 默认语言 |
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 预览默认语言
rosetta-connect preview

# 预览中文版本
rosetta-connect preview --locale zh-Hans

# 预览法文版本
rosetta-connect preview --locale fr-FR
```

#### 输出示例
```
Previewing content for locale: zh-Hans
==================================================
📱 App Name: 我的超棒应用
📝 Subtitle: 最佳生产力工具

📄 Description:
使用我们的创新应用程序改变您的日常工作流程...
(已截断 - 4000 字符限制)

🏷️  Keywords:
生产力,工作流,效率,工具,商业

✨ What's New:
• 错误修复和性能改进
• 新的深色模式主题
• 增强的用户界面

🖼️  Screenshots:
   1. 主界面 (1284x2778)
   2. 设置屏幕 (1284x2778)
   3. 深色模式 (1284x2778)
   4. 功能展示 (1284x2778)

📊 Content Stats:
   • Description: 1,234 / 4,000 字符
   • Keywords: 45 / 100 字符
   • Screenshots: 4 / 10 图片
```

---

### `rosetta-connect validate`
**用途**: 验证内容是否符合 App Store 指南

#### 语法
```bash
rosetta-connect validate [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 基本验证
rosetta-connect validate

# 详细验证
rosetta-connect validate --verbose
```

#### 验证项目
1. **字符限制检查**
   - 应用名称: ≤ 30 字符
   - 副标题: ≤ 30 字符
   - 描述: ≤ 4000 字符
   - 关键词: ≤ 100 字符

2. **内容合规检查**
   - 禁用词汇检测
   - 敏感内容筛查
   - 格式规范验证

3. **截图验证**
   - 尺寸要求检查
   - 数量限制验证
   - 格式支持检查

---

### `rosetta-connect cost`
**用途**: 估算 AI API 调用成本

#### 语法
```bash
rosetta-connect cost [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `--detailed` | Flag | 否 | 显示详细分解 | false |
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 基本成本估算
rosetta-connect cost

# 详细成本分解
rosetta-connect cost --detailed
```

#### 输出示例
```
💰 Cost Estimation
==============================
📊 Breakdown by locale:
   • zh-Hans: $0.15
     - App description: $0.08
     - Keywords: $0.02
     - What's new: $0.05
   • fr-FR: $0.15
     - App description: $0.08
     - Keywords: $0.02
     - What's new: $0.05

📈 Summary:
   • Model: gpt-4o-mini
   • Target locales: 2
   • Estimated cost: $0.30
   • Max cost (with retries): $0.45
```

---

### `rosetta-connect rollback`
**用途**: 回滚到指定版本

#### 语法
```bash
rosetta-connect rollback <VERSION> [OPTIONS]
```

#### 参数
| 参数 | 类型 | 必需 | 说明 | 默认值 |
|------|------|------|------|--------|
| `<VERSION>` | String | 是 | 目标版本号 | 无 |
| `-c, --config <CONFIG>` | Path | 否 | 配置文件路径 | `rosetta.toml` |
| `-v, --verbose` | Flag | 否 | 详细输出 | false |

#### 示例
```bash
# 回滚到指定版本
rosetta-connect rollback 2.3.2

# 详细回滚信息
rosetta-connect rollback 2.3.2 --verbose
```

---

### `rosetta-connect template`
**用途**: 管理 AI 提示模板

#### 语法
```bash
rosetta-connect template <SUBCOMMAND> [OPTIONS]
```

#### 子命令

##### `template list`
列出所有可用模板

```bash
rosetta-connect template list
```

**输出示例:**
```
📋 Available AI prompt templates:
   • app-description    - 生成应用描述
   • keywords          - 生成相关关键词
   • whats-new         - 生成发布说明
   • marketing-copy    - 生成营销文案
   • privacy-policy    - 生成隐私政策片段
```

##### `template create`
创建新模板

```bash
rosetta-connect template create <NAME> --file <FILE>
```

**示例:**
```bash
# 创建新模板
echo "为这个应用生成专业的描述：{app_name}" > my-prompt.txt
rosetta-connect template create my-template --file my-prompt.txt
```

##### `template edit`
编辑现有模板

```bash
rosetta-connect template edit <NAME>
```

##### `template delete`
删除模板

```bash
rosetta-connect template delete <NAME>
```

**示例:**
```bash
# 删除模板 (会有确认提示)
rosetta-connect template delete my-template
```

## 🔧 全局选项

所有命令都支持以下全局选项:

| 选项 | 短形式 | 说明 | 默认值 |
|------|--------|------|--------|
| `--config <CONFIG>` | `-c` | 配置文件路径 | `rosetta.toml` |
| `--verbose` | `-v` | 详细输出模式 | false |
| `--help` | `-h` | 显示帮助信息 | - |
| `--version` | `-V` | 显示版本信息 | - |

## 🚀 常用命令组合

### 日常更新流程
```bash
rosetta-connect pull
rosetta-connect translate
rosetta-connect validate
rosetta-connect push $(date +%Y.%m.%d) --yes
```

### 快速预览流程
```bash
rosetta-connect translate --locales zh-Hans
rosetta-connect preview --locale zh-Hans
rosetta-connect cost --detailed
```

### 发布前检查
```bash
rosetta-connect validate
rosetta-connect diff
rosetta-connect cost
```

## 🌟 使用技巧

### 1. 使用配置文件管理多个项目
```bash
# 开发环境
rosetta-connect pull --config dev.toml

# 生产环境
rosetta-connect push 2.3.4 --config prod.toml
```

### 2. 批量语言处理
```bash
# 一次翻译多种语言
rosetta-connect translate --locales zh-Hans,zh-Hant,ja,ko

# 按地区分组翻译
rosetta-connect translate --locales zh-Hans,zh-Hant  # 中文
rosetta-connect translate --locales fr-FR,de-DE,es-ES  # 欧洲
```

### 3. 成本控制
```bash
# 先估算成本
rosetta-connect cost --detailed

# 使用更便宜的模型
rosetta-connect translate --model gpt-3.5-turbo

# 只翻译核心语言
rosetta-connect translate --locales zh-Hans,ja
```

### 4. 调试和排错
```bash
# 开启详细模式
rosetta-connect pull --verbose

# 开启 JavaScript 调试
ROSETTA_DEBUG_JS=1 rosetta-connect pull
```

## 🆘 常见错误和解决方案

### 配置错误
```bash
Error: Failed to load configuration: rosetta.toml
```
**解决方案**: 确保在项目目录中运行，或使用 `--config` 指定配置文件路径

### 凭据错误
```bash
Error: Invalid API credentials
```
**解决方案**: 检查 `.env` 文件中的凭据是否正确，确保 `.p8` 文件存在

### 网络错误
```bash
Error: Failed to connect to API
```
**解决方案**: 检查网络连接，确认 API 服务可用

---

<p align="center">
  💡 <strong>更多使用技巧请查看</strong> → <a href="./best-practices.md">最佳实践指南</a>
</p>