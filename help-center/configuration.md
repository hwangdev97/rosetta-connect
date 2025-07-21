# ⚙️ 配置详解

本文档详细介绍 Rosetta Connect 的所有配置选项，帮助你自定义工具以满足特定需求。

## 📁 配置文件概览

Rosetta Connect 使用两个主要配置文件：

| 文件 | 用途 | 格式 | 版本控制 |
|------|------|------|----------|
| `rosetta.toml` | 项目配置 | TOML | ✅ 建议提交 |
| `.env` | 敏感凭据 | Shell 环境变量 | ❌ 不要提交 |

## 📝 rosetta.toml 详解

### 基本结构
```toml
[app]          # 应用基本信息
[assets]       # 资源文件配置  
[ai]           # AI 翻译设置
[advanced]     # 高级选项 (可选)
```

### [app] 节 - 应用配置

#### 必需选项
```toml
[app]
bundle_id = "com.example.myapp"        # App Bundle ID
default_locale = "en-US"               # 默认语言区域
target_locales = ["zh-Hans", "fr-FR"] # 目标翻译语言列表
```

#### 可选选项
```toml
[app]
# 应用分类 (用于优化 AI 翻译上下文)
category = "productivity"              # productivity, games, social, etc.

# 应用简短描述 (帮助 AI 理解应用用途)
description = "A powerful note-taking app"

# 目标用户群体
target_audience = "professionals"     # professionals, students, general, etc.

# 应用版本管理
version_format = "YYYY.MM.DD"         # 版本号格式
auto_increment = true                  # 自动递增版本号
```

**语言代码参考:**
| 语言 | 代码 | 语言 | 代码 |
|------|------|------|------|
| 英语(美国) | `en-US` | 中文简体 | `zh-Hans` |
| 中文繁体 | `zh-Hant` | 日语 | `ja` |
| 韩语 | `ko` | 法语 | `fr-FR` |
| 德语 | `de-DE` | 西班牙语 | `es-ES` |
| 意大利语 | `it` | 葡萄牙语 | `pt-BR` |
| 俄语 | `ru` | 阿拉伯语 | `ar` |
| 荷兰语 | `nl-NL` | 瑞典语 | `sv` |
| 挪威语 | `no` | 丹麦语 | `da` |
| 芬兰语 | `fi` | 波兰语 | `pl` |

### [assets] 节 - 资源配置

#### 截图路径配置
```toml
[assets]
# 默认截图目录 (通常是英语)
default = "./screenshots/en"

# 特定语言的截图目录
zh-Hans = "./screenshots/zh"
ja = "./screenshots/ja"
fr-FR = "./screenshots/fr"

# 可以使用相对路径或绝对路径
# 相对路径基于 rosetta.toml 文件位置
```

#### 高级资源配置
```toml
[assets]
# 截图尺寸验证
validate_dimensions = true             # 验证截图尺寸
required_sizes = ["1284x2778", "2778x1284"]  # 必需的尺寸

# 文件格式要求
allowed_formats = ["png", "jpg"]       # 允许的文件格式
max_file_size = "10MB"                 # 最大文件大小

# 命名规范
naming_pattern = "{locale}_{index}.{ext}"  # 文件命名模式
```

### [ai] 节 - AI 翻译配置

#### 基本 AI 设置
```toml
[ai]
# AI 服务提供商
provider = "openai"                    # 当前仅支持 openai

# 使用的 AI 模型
model = "gpt-4o-mini"                  # 推荐的平衡选择

# 生成参数
temperature = 0.7                      # 0.0-2.0, 控制创造性
max_tokens = 1024                      # 每次生成的最大 token 数
```

#### 模型选择指南
| 模型 | 速度 | 质量 | 成本 | 适用场景 |
|------|------|------|------|----------|
| `gpt-4o` | 中 | 最高 | 高 | 重要产品发布 |
| `gpt-4o-mini` | 快 | 高 | 中 | 日常翻译工作 |
| `gpt-4-turbo` | 中 | 最高 | 最高 | 专业内容翻译 |
| `gpt-3.5-turbo` | 最快 | 中 | 低 | 快速原型测试 |

#### 高级 AI 配置
```toml
[ai]
# 重试机制
max_retries = 3                        # API 调用失败时的重试次数
retry_delay = 1000                     # 重试延迟 (毫秒)

# 并发控制
max_concurrent_requests = 3            # 最大并发请求数
request_timeout = 30000                # 请求超时 (毫秒)

# 质量控制
enable_validation = true               # 启用翻译结果验证
min_quality_score = 0.8               # 最低质量评分

# 成本控制
daily_cost_limit = 10.00              # 每日成本限制 (美元)
monthly_cost_limit = 100.00           # 每月成本限制 (美元)
```

### [advanced] 节 - 高级选项

```toml
[advanced]
# 缓存设置
enable_cache = true                    # 启用翻译缓存
cache_ttl = 86400                      # 缓存有效期 (秒)
cache_dir = "~/.rosetta-cache"         # 缓存目录

# 日志配置
log_level = "info"                     # debug, info, warn, error
log_file = "./logs/rosetta.log"        # 日志文件路径
enable_file_logging = false           # 启用文件日志

# 网络设置
connect_timeout = 10000                # 连接超时 (毫秒)
read_timeout = 30000                   # 读取超时 (毫秒)
user_agent = "RosettaConnect/1.0"      # User-Agent 字符串

# 调试选项
debug_mode = false                     # 调试模式
save_requests = false                  # 保存 API 请求日志
verbose_errors = true                  # 详细错误信息
```

## 🔑 .env 环境变量

### App Store Connect API 凭据
```bash
# Apple 开发者账号信息
ISSUER_ID=12345678-1234-1234-1234-123456789abc    # 发行者 ID
KEY_ID=ABCD123456                                  # 密钥 ID
PRIVATE_KEY_PATH=./AuthKey_ABCD123456.p8          # 私钥文件路径

# 可选: 团队信息
TEAM_ID=XXXXXXXXXX                                # 团队 ID (如果需要)
```

### OpenAI API 配置
```bash
# OpenAI API 密钥
OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# 可选: OpenAI 组织 ID
OPENAI_ORGANIZATION=org-xxxxxxxxxxxxxxxx
```

### 高级环境变量
```bash
# 代理设置 (如果需要)
HTTP_PROXY=http://proxy.company.com:8080
HTTPS_PROXY=http://proxy.company.com:8080
NO_PROXY=localhost,127.0.0.1

# 调试设置
ROSETTA_DEBUG=1                        # 启用调试模式
ROSETTA_DEBUG_JS=1                     # 启用 JS 桥接调试
ROSETTA_LOG_LEVEL=debug               # 覆盖日志级别

# 缓存设置
ROSETTA_CACHE_DIR=/tmp/rosetta-cache  # 自定义缓存目录
ROSETTA_NO_CACHE=1                    # 禁用缓存

# 性能调优
ROSETTA_MAX_WORKERS=4                 # 最大工作线程数
ROSETTA_MEMORY_LIMIT=512M             # 内存限制
```

## 📋 配置模板

### 基础模板
```toml
# rosetta.toml - 基础配置模板

[app]
bundle_id = "com.example.myapp"
default_locale = "en-US"
target_locales = ["zh-Hans", "fr-FR", "de-DE"]
category = "productivity"
description = "A powerful productivity app"

[assets]
default = "./screenshots/en"
zh-Hans = "./screenshots/zh"
fr-FR = "./screenshots/fr"
de-DE = "./screenshots/de"

[ai]
provider = "openai"
model = "gpt-4o-mini"
temperature = 0.7
max_tokens = 1024
```

### 多环境模板

#### 开发环境 (development.toml)
```toml
[app]
bundle_id = "com.example.myapp.dev"
default_locale = "en-US"
target_locales = ["zh-Hans"]          # 只测试一种语言

[ai]
provider = "openai"
model = "gpt-3.5-turbo"               # 使用便宜的模型
temperature = 0.3                     # 更一致的结果
max_tokens = 512                      # 降低成本

[advanced]
debug_mode = true
save_requests = true
log_level = "debug"
```

#### 生产环境 (production.toml)
```toml
[app]
bundle_id = "com.example.myapp"
default_locale = "en-US"
target_locales = [
    "zh-Hans", "zh-Hant", "ja", "ko",
    "fr-FR", "de-DE", "es-ES", "pt-BR",
    "ru", "it", "nl-NL"
]

[ai]
provider = "openai"
model = "gpt-4o"                      # 最佳质量
temperature = 0.7
max_tokens = 2048

[advanced]
enable_cache = true
daily_cost_limit = 50.00
log_level = "info"
```

### 团队协作模板
```toml
# team-shared.toml - 团队共享配置

[app]
bundle_id = "com.company.product"
default_locale = "en-US"
# 按优先级分组的目标语言
target_locales = [
    # Tier 1: 核心市场
    "zh-Hans", "ja", "ko",
    # Tier 2: 欧美市场  
    "fr-FR", "de-DE", "es-ES",
    # Tier 3: 扩展市场
    "zh-Hant", "pt-BR", "ru", "it"
]

# 应用元信息 (帮助 AI 翻译)
category = "business"
description = "Enterprise collaboration platform"
target_audience = "business professionals"

[assets]
# 标准化的目录结构
default = "./assets/screenshots/en"
zh-Hans = "./assets/screenshots/zh-cn"
zh-Hant = "./assets/screenshots/zh-tw"
ja = "./assets/screenshots/ja"
ko = "./assets/screenshots/ko"
fr-FR = "./assets/screenshots/fr"
de-DE = "./assets/screenshots/de"
es-ES = "./assets/screenshots/es"
pt-BR = "./assets/screenshots/pt"
ru = "./assets/screenshots/ru"
it = "./assets/screenshots/it"

[ai]
provider = "openai"
model = "gpt-4o-mini"                 # 平衡成本和质量
temperature = 0.7
max_tokens = 1024

# 成本控制
max_concurrent_requests = 2           # 避免限流
daily_cost_limit = 20.00             # 团队预算控制

[advanced]
enable_cache = true
cache_ttl = 86400                     # 24小时缓存
log_level = "info"
```

## 🔧 配置验证

### 配置检查命令
```bash
# 验证配置文件语法
rosetta-connect validate --config-only

# 检查环境变量
rosetta-connect validate --env-only

# 完整配置验证
rosetta-connect validate
```

### 常见配置错误

#### TOML 格式错误
```toml
# ❌ 错误的格式
temperature = "0.7"                   # 数字不应该用引号
target_locales = zh-Hans, fr-FR       # 数组格式错误

# ✅ 正确的格式
temperature = 0.7                     # 数字类型
target_locales = ["zh-Hans", "fr-FR"] # 字符串数组
```

#### 路径配置错误
```toml
# ❌ 路径错误
default = "screenshots/en"            # 相对路径可能有问题

# ✅ 推荐写法
default = "./screenshots/en"          # 明确的相对路径
# 或
default = "/absolute/path/screenshots/en"  # 绝对路径
```

## 🚀 配置优化建议

### 性能优化
```toml
[ai]
# 批量处理优化
max_concurrent_requests = 3           # 根据 API 限制调整
request_timeout = 30000               # 适当的超时设置

[advanced]
# 缓存优化
enable_cache = true
cache_ttl = 86400                     # 根据内容更新频率调整
```

### 成本优化
```toml
[ai]
model = "gpt-4o-mini"                 # 性价比最佳
max_tokens = 1024                     # 控制输出长度
temperature = 0.3                     # 更确定的结果，减少重试

# 成本限制
daily_cost_limit = 10.00
monthly_cost_limit = 100.00
```

### 质量优化
```toml
[app]
# 提供更多上下文信息
category = "productivity"
description = "Task management and note-taking app"
target_audience = "knowledge workers"

[ai]
model = "gpt-4o"                      # 对重要内容使用最佳模型
temperature = 0.7                     # 平衡创造性和准确性
```

## 📚 配置参考

### 完整参数列表

#### [app] 节参数
| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `bundle_id` | String | ✅ | - | App Store Bundle ID |
| `default_locale` | String | ✅ | `"en-US"` | 默认语言区域 |
| `target_locales` | Array | ✅ | `[]` | 目标翻译语言 |
| `category` | String | ❌ | - | 应用分类 |
| `description` | String | ❌ | - | 应用描述 |
| `target_audience` | String | ❌ | - | 目标用户群体 |
| `version_format` | String | ❌ | `"YYYY.MM.DD"` | 版本号格式 |

#### [ai] 节参数
| 参数 | 类型 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `provider` | String | ❌ | `"openai"` | AI 服务提供商 |
| `model` | String | ❌ | `"gpt-4o-mini"` | AI 模型名称 |
| `temperature` | Float | ❌ | `0.7` | 生成温度 (0.0-2.0) |
| `max_tokens` | Integer | ❌ | `1024` | 最大 token 数 |
| `max_retries` | Integer | ❌ | `3` | 最大重试次数 |
| `request_timeout` | Integer | ❌ | `30000` | 请求超时 (毫秒) |

#### 环境变量列表
| 变量名 | 必需 | 说明 |
|--------|------|------|
| `ISSUER_ID` | ✅ | App Store Connect Issuer ID |
| `KEY_ID` | ✅ | App Store Connect Key ID |
| `PRIVATE_KEY_PATH` | ✅ | 私钥文件路径 |
| `OPENAI_API_KEY` | ❌ | OpenAI API 密钥 |
| `ROSETTA_DEBUG` | ❌ | 启用调试模式 |
| `HTTP_PROXY` | ❌ | HTTP 代理设置 |

---

<p align="center">
  ⚙️ <strong>配置优化是提升效率的关键</strong>
</p>

<p align="center">
  需要更多配置帮助？查看 <a href="./troubleshooting.md">故障排除</a> 或 <a href="./best-practices.md">最佳实践</a>
</p>