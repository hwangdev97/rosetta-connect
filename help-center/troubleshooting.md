# 🔧 故障排除指南

这个指南帮助你快速诊断和解决使用 Rosetta Connect 时遇到的常见问题。

## 🚨 常见问题分类

### 📦 安装和环境问题
- [Rust 编译错误](#rust-编译错误)
- [依赖项缺失](#依赖项缺失)
- [权限问题](#权限问题)

### 🔑 配置和凭据问题  
- [App Store Connect API 错误](#app-store-connect-api-错误)
- [OpenAI API 错误](#openai-api-错误)
- [配置文件问题](#配置文件问题)

### 🌐 网络和连接问题
- [网络连接失败](#网络连接失败)
- [代理配置](#代理配置)
- [API 限流](#api-限流)

### 🤖 AI 翻译问题
- [翻译质量问题](#翻译质量问题)
- [成本控制问题](#成本控制问题)
- [模型选择问题](#模型选择问题)

### 📱 App Store Connect 问题
- [上传失败](#上传失败)
- [截图问题](#截图问题)
- [元数据验证失败](#元数据验证失败)

## 🔍 快速诊断

### 第一步：检查基本信息
```bash
# 检查工具版本
rosetta-connect --version

# 检查配置文件
cat rosetta.toml

# 检查环境变量
env | grep -E "(ISSUER_ID|KEY_ID|OPENAI_API_KEY)"
```

### 第二步：开启详细模式
```bash
# 使用详细输出模式
rosetta-connect --verbose <command>

# 开启 JavaScript 调试 (未来功能)
ROSETTA_DEBUG_JS=1 rosetta-connect <command>
```

### 第三步：检查网络连接
```bash
# 测试 OpenAI API 连接
curl -H "Authorization: Bearer $OPENAI_API_KEY" \
     "https://api.openai.com/v1/models"

# 测试 App Store Connect API 连接
curl "https://api.appstoreconnect.apple.com/v1/apps"
```

## 🛠️ 具体问题解决方案

### Rust 编译错误

#### 问题描述
```
error[E0599]: no method named `create_string` found for struct `Env`
```

#### 解决方案
1. **更新 Rust 版本**
   ```bash
   rustup update stable
   rustc --version  # 确保 >= 1.70.0
   ```

2. **清理构建缓存**
   ```bash
   cargo clean
   cargo build --release
   ```

3. **检查依赖版本**
   ```bash
   cargo tree  # 查看依赖树
   cargo update  # 更新依赖
   ```

#### 预防措施
- 使用 `rustup` 管理 Rust 版本
- 定期更新工具链
- 使用 `Cargo.lock` 锁定依赖版本

---

### 依赖项缺失

#### 问题描述
```
Package `pkg-config` was not found
```

#### 解决方案

**macOS:**
```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 使用 Homebrew 安装依赖
brew install pkg-config openssl
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install pkg-config libssl-dev
```

**Linux (CentOS/RHEL):**
```bash
sudo yum install pkg-config openssl-devel
```

---

### 权限问题

#### 问题描述
```
Permission denied (os error 13)
```

#### 解决方案
1. **检查文件权限**
   ```bash
   ls -la rosetta.toml
   ls -la .env
   ls -la AuthKey_*.p8
   ```

2. **修复权限**
   ```bash
   chmod 644 rosetta.toml
   chmod 600 .env
   chmod 600 AuthKey_*.p8  # 私钥文件必须是 600
   ```

3. **检查目录权限**
   ```bash
   ls -la screenshots/
   chmod 755 screenshots/*/
   ```

---

### App Store Connect API 错误

#### 问题描述
```
Error: Invalid API credentials
Error: JWT token expired
```

#### 解决方案
1. **验证凭据格式**
   ```bash
   # 检查 ISSUER_ID 格式 (UUID)
   echo $ISSUER_ID | grep -E "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"
   
   # 检查 KEY_ID 格式 (10个字符)
   echo $KEY_ID | grep -E "^[A-Z0-9]{10}$"
   
   # 检查私钥文件
   head -1 AuthKey_*.p8  # 应该显示 "-----BEGIN PRIVATE KEY-----"
   ```

2. **重新生成 API 密钥**
   - 登录 [App Store Connect](https://appstoreconnect.apple.com)
   - 进入"用户和访问权限" → "密钥"
   - 撤销旧密钥，生成新密钥
   - 更新 `.env` 文件中的凭据

3. **检查密钥权限**
   - 确保密钥有"开发者"或"管理"权限
   - 验证密钥未过期
   - 确认密钥对应正确的团队

#### 常见错误码
| 错误码 | 含义 | 解决方案 |
|--------|------|----------|
| 401 | 未授权 | 检查 API 凭据 |
| 403 | 权限不足 | 升级密钥权限 |
| 429 | 请求过多 | 等待后重试 |
| 500 | 服务器错误 | 联系 Apple 支持 |

---

### OpenAI API 错误

#### 问题描述
```
Error: You exceeded your current quota
Error: The model `gpt-4` does not exist
```

#### 解决方案
1. **检查账户余额**
   ```bash
   curl -H "Authorization: Bearer $OPENAI_API_KEY" \
        "https://api.openai.com/v1/usage"
   ```

2. **验证模型可用性**
   ```bash
   curl -H "Authorization: Bearer $OPENAI_API_KEY" \
        "https://api.openai.com/v1/models" | \
        jq '.data[] | select(.id | contains("gpt"))'
   ```

3. **使用可用模型**
   ```toml
   # 在 rosetta.toml 中使用确定可用的模型
   [ai]
   model = "gpt-3.5-turbo"  # 基础模型，通常都可用
   ```

4. **成本控制**
   ```bash
   # 先估算成本
   rosetta-connect cost --detailed
   
   # 使用更便宜的模型
   rosetta-connect translate --model gpt-3.5-turbo
   ```

---

### 配置文件问题

#### 问题描述
```
Error: Failed to load configuration: rosetta.toml
Error: invalid type: string "0.7", expected f32
```

#### 解决方案
1. **验证 TOML 语法**
   ```bash
   # 使用在线 TOML 验证器或
   python3 -c "import toml; print(toml.load('rosetta.toml'))"
   ```

2. **修复常见格式错误**
   ```toml
   # ❌ 错误格式
   temperature = "0.7"      # 字符串
   target_locales = zh-Hans # 缺少引号和数组
   
   # ✅ 正确格式  
   temperature = 0.7        # 数字
   target_locales = ["zh-Hans", "fr-FR"]  # 字符串数组
   ```

3. **使用配置模板**
   ```bash
   # 重新初始化以获得正确的配置模板
   mv rosetta.toml rosetta.toml.backup
   rosetta-connect init --bundle-id com.example.myapp
   ```

---

### 网络连接失败

#### 问题描述
```
Error: Connection timeout
Error: SSL certificate verification failed
```

#### 解决方案
1. **检查网络连接**
   ```bash
   ping api.openai.com
   ping api.appstoreconnect.apple.com
   ```

2. **检查防火墙设置**
   - 确保允许 HTTPS (443) 出站连接
   - 检查企业防火墙规则
   - 验证 DNS 解析

3. **配置代理**
   ```bash
   # 设置 HTTP 代理
   export HTTP_PROXY=http://proxy.company.com:8080
   export HTTPS_PROXY=http://proxy.company.com:8080
   
   # 或在 ~/.cargo/config.toml 中配置
   [http]
   proxy = "http://proxy.company.com:8080"
   ```

4. **SSL 证书问题**
   ```bash
   # 更新 CA 证书
   # macOS
   brew install ca-certificates
   
   # Linux
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

---

### API 限流

#### 问题描述
```
Error: Too Many Requests (429)
Retry-After: 60
```

#### 解决方案
1. **实施退避策略**
   ```bash
   # 等待指定时间后重试
   sleep 60
   rosetta-connect translate
   ```

2. **减少并发请求**
   ```toml
   # 在配置中限制并发数
   [ai]
   max_concurrent_requests = 2  # 未来功能
   ```

3. **分批处理**
   ```bash
   # 分别翻译不同语言
   rosetta-connect translate --locales zh-Hans
   sleep 30
   rosetta-connect translate --locales fr-FR
   ```

---

### 翻译质量问题

#### 问题描述
- 翻译不准确
- 术语不一致  
- 文化适应性差

#### 解决方案
1. **优化提示词**
   ```bash
   # 创建自定义模板
   rosetta-connect template create app-desc-cn --file chinese-prompt.txt
   ```

2. **提供更多上下文**
   ```toml
   # 在配置中添加应用信息
   [app]
   category = "productivity"
   description = "A note-taking app for professionals"
   ```

3. **使用更好的模型**
   ```bash
   # 对重要内容使用 GPT-4
   rosetta-connect translate --model gpt-4 --locales zh-Hans
   ```

4. **人工审核流程**
   ```bash
   # 先生成翻译
   rosetta-connect translate
   
   # 预览检查
   rosetta-connect preview --locale zh-Hans
   
   # 手动编辑后推送
   rosetta-connect push 2.3.4
   ```

---

### 上传失败

#### 问题描述
```
Error: Screenshot upload failed
Error: Metadata validation failed
```

#### 解决方案
1. **检查文件格式**
   ```bash
   # 截图要求
   file screenshots/en/*.png  # 应该是 PNG 格式
   
   # 检查尺寸
   identify screenshots/en/*.png | grep "1284x2778"
   ```

2. **验证内容长度**
   ```bash
   # 检查描述长度
   rosetta-connect validate
   ```

3. **分步上传**
   ```bash
   # 先只上传元数据
   rosetta-connect push 2.3.4 --metadata-only  # 未来功能
   
   # 然后上传截图
   rosetta-connect push 2.3.4 --screenshots-only  # 未来功能
   ```

---

## 🆘 获取更多帮助

### 自助诊断工具
```bash
# 创建诊断脚本
cat > diagnose.sh << 'EOF'
#!/bin/bash
echo "=== Rosetta Connect 诊断报告 ==="
echo "时间: $(date)"
echo "系统: $(uname -a)"
echo
echo "=== 版本信息 ==="
rosetta-connect --version
cargo --version
rustc --version
echo
echo "=== 配置检查 ==="
ls -la rosetta.toml .env AuthKey_*.p8 2>/dev/null || echo "某些配置文件缺失"
echo
echo "=== 网络测试 ==="
ping -c 1 api.openai.com && echo "OpenAI API 可达" || echo "OpenAI API 不可达"
ping -c 1 api.appstoreconnect.apple.com && echo "App Store API 可达" || echo "App Store API 不可达"
echo
echo "=== 环境变量 ==="
env | grep -E "(ISSUER_ID|KEY_ID|OPENAI_API_KEY)" | sed 's/=.*/=***/'
EOF

chmod +x diagnose.sh
./diagnose.sh
```

### 在线资源
- 🐛 **提交 Bug**: [GitHub Issues](https://github.com/yourusername/rosetta-connect/issues)
- 💬 **社区讨论**: [GitHub Discussions](https://github.com/yourusername/rosetta-connect/discussions)
- 📚 **文档中心**: [Help Center](./README.md)
- 🔍 **搜索问题**: 使用 GitHub 搜索功能查找类似问题

### 提交问题时请包含
1. **系统信息**: 操作系统、版本、架构
2. **工具版本**: `rosetta-connect --version`
3. **完整错误信息**: 包含堆栈跟踪
4. **重现步骤**: 具体的操作步骤
5. **配置信息**: 去除敏感信息的配置文件
6. **诊断报告**: 运行上面的诊断脚本

### 紧急联系方式
- **邮箱**: rosetta-connect@example.com
- **Discord**: [加入服务器](https://discord.gg/rosetta-connect)
- **微信群**: 扫描二维码加入

## 📋 问题报告模板

```markdown
## 问题描述
[简要描述遇到的问题]

## 环境信息
- 操作系统: 
- Rosetta Connect 版本: 
- Rust 版本: 

## 重现步骤
1. 
2. 
3. 

## 预期行为
[描述期望的正确行为]

## 实际行为
[描述实际发生的情况]

## 错误信息
```
[粘贴完整的错误信息]
```

## 额外信息
[任何可能相关的额外信息]
```

## 🔮 常见问题预防

### 开发环境最佳实践
1. **使用版本管理**
   ```bash
   # 锁定 Rust 版本
   rustup override set 1.75.0
   
   # 提交 Cargo.lock
   git add Cargo.lock
   ```

2. **环境隔离**
   ```bash
   # 使用项目特定的环境
   cp .env.example .env
   # 编辑 .env 而不是修改系统环境变量
   ```

3. **定期更新**
   ```bash
   # 定期更新工具
   git pull origin main
   cargo build --release
   
   # 检查新版本
   rosetta-connect --version
   ```

### 生产环境检查清单
- [ ] API 凭据正确配置
- [ ] 网络连接稳定
- [ ] 权限设置正确
- [ ] 备份重要配置
- [ ] 监控 API 配额
- [ ] 测试翻译质量

---

<p align="center">
  🔧 <strong>还有问题?</strong> 查看 <a href="./best-practices.md">最佳实践</a> 或 <a href="https://github.com/yourusername/rosetta-connect/discussions">加入讨论</a>
</p>