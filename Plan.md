# agent-pet-rs 2.0 - AI 提供商适配器系统

## 项目概述

agent-pet-rs 是一个智能桌面宠物框架。本计划实现完整的 AI 提供商适配器系统，支持多提供商、自动切换、Token 计费、速率限制、预算管理等功能。

## 当前状态

**版本**: 2.0.0-alpha
**完成度**: 核心框架 100% + TUI 宠物 100% + AI 适配器计划 100%

## 核心特性

### 已确认需求

| 特性 | 状态 | 优先级 |
|------|------|--------|
| 多提供商支持 (11个) | ✅ 确认 | 高 |
| 自动切换 (可配置开关和顺序) | ✅ 确认 | 高 |
| 默认模型推荐 | ✅ 确认 | 高 |
| 连接测试 (可选，不强制) | ✅ 确认 | 中 |
| 速率限制 (请求+Token) | ✅ 确认 | 高 |
| Token 计费 (实时显示) | ✅ 确认 | 高 |
| 手动切换提供商 | ✅ 确认 | 高 |
| 手动切换模型 | ✅ 确认 | 高 |
| 加密存储 API Key | ✅ 确认 | 高 |
| 价格配置文件覆盖 | ✅ 确认 | 中 |
| 使用量导出 (JSON/CSV) | ✅ 确认 | 中 |
| 预算限制 (每日/每月) | ✅ 确认 | 高 |
| i18n 支持 | 🔄 未来 | 低 |

## 架构设计

### 文件结构

```
examples/pet_agent/
├── Cargo.toml
└── src/
    ├── main.rs              # 主入口
    ├── app.rs               # 应用状态
    ├── ui.rs                # TUI 渲染
    ├── event.rs             # 事件处理
    ├── config.rs            # 配置管理
    ├── location.rs          # 位置系统
    ├── pet.rs               # 小狗显示
    ├── animation.rs         # 动画系统
    ├── commands.rs          # 命令处理
    ├── setup.rs             # 首次配置
    └── ai/
        ├── mod.rs           # 模块定义
        ├── provider.rs      # 提供商接口
        ├── manager.rs       # 提供商管理器
        ├── pricing.rs       # Token 计费
        ├── rate_limiter.rs  # 速率限制
        ├── usage.rs         # 使用量追踪
        ├── budget.rs        # 预算管理
        ├── crypto.rs        # API Key 加密
        ├── error.rs         # 错误类型
        └── adapters/
            ├── mod.rs       # 适配器注册
            ├── openrouter.rs
            ├── openai.rs
            ├── anthropic.rs
            ├── google.rs
            ├── mistral.rs
            ├── cohere.rs
            ├── groq.rs
            ├── together.rs
            ├── ollama.rs
            ├── lmstudio.rs
            └── custom.rs
```

## 支持的提供商

| 提供商 | 名称 | 默认模型 | API 端点 | 需要 API Key |
|--------|------|----------|----------|--------------|
| OpenRouter | openrouter | google/gemma-3-27b-it:free | https://openrouter.ai/api/v1 | ✅ |
| OpenAI | openai | gpt-4o-mini | https://api.openai.com/v1 | ✅ |
| Anthropic | anthropic | claude-3-haiku-20240307 | https://api.anthropic.com/v1 | ✅ |
| Google | google | gemini-1.5-flash | https://generativelanguage.googleapis.com/v1 | ✅ |
| Mistral | mistral | mistral-small-latest | https://api.mistral.ai/v1 | ✅ |
| Cohere | cohere | command-r | https://api.cohere.ai/v1 | ✅ |
| Groq | groq | llama3-8b-8192 | https://api.groq.com/openai/v1 | ✅ |
| Together | together | meta-llama/Llama-3-8b-chat-hf | https://api.together.xyz/v1 | ✅ |
| Ollama | ollama | llama3 | http://localhost:11434 | ❌ |
| LM Studio | lmstudio | (自动) | http://localhost:1234/v1 | ❌ |
| Custom | custom | - | 用户指定 | 可选 |

## Token 价格表

| 提供商 | 模型 | 输入价格 | 输出价格 | 单位 |
|--------|------|----------|----------|------|
| OpenRouter | gemma-3-27b-free | $0.00 | $0.00 | /1M tokens |
| OpenRouter | llama-3-70b | $0.50 | $0.50 | /1M tokens |
| OpenAI | gpt-4o | $2.50 | $10.00 | /1M tokens |
| OpenAI | gpt-4o-mini | $0.15 | $0.60 | /1M tokens |
| Anthropic | claude-3-opus | $15.00 | $75.00 | /1M tokens |
| Anthropic | claude-3-sonnet | $3.00 | $15.00 | /1M tokens |
| Anthropic | claude-3-haiku | $0.25 | $1.25 | /1M tokens |
| Google | gemini-1.5-pro | $3.50 | $10.50 | /1M tokens |
| Google | gemini-1.5-flash | $0.075 | $0.30 | /1M tokens |
| Ollama | * | $0.00 | $0.00 | 本地免费 |

## 配置文件格式

```toml
# ~/.pet_agent/config.toml

[settings]
memory_path = "~/.pet_agent/memory.json"
window_width = 80
window_height = 24
animation_speed = 200
dog_size = "Medium"

[ai]
auto_switch = true
switch_order = ["openrouter", "openai", "ollama"]

[ai.pricing.overrides]
"openrouter/google/gemma-3-27b-it:free" = { input = 0.0, output = 0.0 }
"openai/gpt-4o-mini" = { input = 0.15, output = 0.60 }

[ai.budget]
enabled = true
daily_limit = 5.00
monthly_limit = 50.00
warning_threshold = 0.8

[[ai.providers]]
name = "openrouter"
enabled = true
api_key_encrypted = "U2FsdGVkX1..."
model = "google/gemma-3-27b-it:free"
api_base = "https://openrouter.ai/api/v1"
max_tokens = 1000
temperature = 0.7
priority = 1

[ai.providers.rate_limit]
enabled = true
requests_per_minute = 20
requests_per_hour = 200
tokens_per_minute = 40000
tokens_per_hour = 400000

[[ai.providers]]
name = "openai"
enabled = true
api_key_encrypted = "U2FsdGVkX1..."
model = "gpt-4o-mini"
api_base = "https://api.openai.com/v1"
max_tokens = 1000
temperature = 0.7
priority = 2

[ai.providers.rate_limit]
enabled = true
requests_per_minute = 60
requests_per_hour = 1000
tokens_per_minute = 100000
tokens_per_hour = 1000000

[[ai.providers]]
name = "ollama"
enabled = true
api_key_encrypted = ""
model = "llama3"
api_base = "http://localhost:11434"
max_tokens = 1000
temperature = 0.7
priority = 3

[ai.providers.rate_limit]
enabled = false
```

## API Key 加密说明

### 存储格式
```
原始 API Key → XOR 加密 → Base64 编码 → 存储到配置文件
```

### 安全建议
1. 设置文件权限: `chmod 600 ~/.pet_agent/config.toml`
2. 不要提交到 Git
3. 这是简单混淆，不是高安全性加密

### 手动处理
```bash
# 加密
echo -n "sk-your-api-key" | openssl enc -aes-256-cbc -a -salt -pass pass:pet-agent

# 解密
echo "encrypted-base64-string" | openssl enc -aes-256-cbc -d -a -pass pass:pet-agent
```

## UI 设计

### 主界面实时显示
```
┌─────────────────────────────────────────────────────────────┐
│  🐕 Buddy - 智能宠物助手                                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  [🏠 屋里]  [🔧 工作室]  [🌳 前院]  [🌾 后院农场]             │
│                                                             │
│  ┌─────────────────────┐    ┌─────────────────────────────┐ │
│  │      /\_/\          │    │  Provider: openrouter       │ │
│  │     ( o.o )         │    │  Model: gemma-3-27b-free    │ │
│  │      > ^ <          │    │  Rate: 5/20 req/min         │ │
│  │     /|   |\         │    │  Cost: $0.0023              │ │
│  │    (_|   |_)        │    ├─────────────────────────────┤ │
│  │                     │    │ [你]: 你好！                 │ │
│  └─────────────────────┘    │ [Buddy]: 汪汪！你好！         │ │
│                             │ [你]: 今天天气怎么样？        │ │
│                             │ [Buddy]: 让我看看...          │ │
│                             │ 📊 +150 tokens ($0.0002)     │ │
│                             └─────────────────────────────┘ │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  输入消息: _________________________________                 │
│  [F1] 喂食  [F2] 玩耍  [F3] 休息  [F4] 探索  [Tab] 位置     │
└─────────────────────────────────────────────────────────────┘
```

## 命令系统

### 提供商命令
```
/provider list           - 显示所有提供商
/provider switch <name>  - 切换提供商
/provider test           - 测试所有连接
/provider set <name> <model> - 设置提供商模型
```

### 模型命令
```
/model list              - 显示当前提供商模型
/model list all          - 显示所有模型
/model switch <name>     - 切换模型
/model info <name>       - 显示模型详情和价格
```

### 使用量命令
```
/stats                   - 显示使用量统计
/export [format]         - 导出使用量 (json/csv)
```

### 预算命令
```
/budget                  - 显示预算状态
/budget set daily <amount>  - 设置每日预算
/budget set monthly <amount> - 设置每月预算
```

## 实施阶段

### Phase 1: 核心基础设施 ⏳ 待实现
- [ ] 创建 `src/ai/mod.rs` - 模块定义
- [ ] 创建 `src/ai/provider.rs` - 提供商接口
- [ ] 创建 `src/ai/error.rs` - 错误类型

**预计时间**: 45 分钟

### Phase 2: 计费系统 ⏳ 待实现
- [ ] 创建 `src/ai/pricing.rs` - Token 计费
- [ ] 实现价格表
- [ ] 实现价格计算

**预计时间**: 30 分钟

### Phase 3: 速率限制 ⏳ 待实现
- [ ] 创建 `src/ai/rate_limiter.rs` - 速率限制器
- [ ] 实现请求限制
- [ ] 实现 Token 限制
- [ ] 实现自动等待

**预计时间**: 25 分钟

### Phase 4: 使用量追踪 ⏳ 待实现
- [ ] 创建 `src/ai/usage.rs` - 使用量追踪
- [ ] 实现记录存储
- [ ] 实现统计计算
- [ ] 实现导出功能

**预计时间**: 35 分钟

### Phase 5: 预算管理 ⏳ 待实现
- [ ] 创建 `src/ai/budget.rs` - 预算管理
- [ ] 实现每日预算
- [ ] 实现每月预算
- [ ] 实现预算警告

**预计时间**: 25 分钟

### Phase 6: 加密工具 ⏳ 待实现
- [ ] 创建 `src/ai/crypto.rs` - API Key 加密
- [ ] 实现 XOR 加密
- [ ] 实现 Base64 编码

**预计时间**: 15 分钟

### Phase 7: 提供商适配器 ⏳ 待实现
- [ ] 创建 `src/ai/adapters/mod.rs`
- [ ] 实现 OpenRouter 适配器
- [ ] 实现 OpenAI 适配器
- [ ] 实现 Anthropic 适配器
- [ ] 实现 Google 适配器
- [ ] 实现 Mistral 适配器
- [ ] 实现 Cohere 适配器
- [ ] 实现 Groq 适配器
- [ ] 实现 Together 适配器
- [ ] 实现 Ollama 适配器
- [ ] 实现 LM Studio 适配器
- [ ] 实现 Custom 适配器

**预计时间**: 90 分钟

### Phase 8: 提供商管理器 ⏳ 待实现
- [ ] 创建 `src/ai/manager.rs` - 提供商管理器
- [ ] 实现自动切换
- [ ] 实现手动切换
- [ ] 实现速率限制集成
- [ ] 实现使用量集成
- [ ] 实现预算集成

**预计时间**: 45 分钟

### Phase 9: 配置系统更新 ⏳ 待实现
- [ ] 更新 `src/config.rs`
- [ ] 添加 AI 配置结构
- [ ] 添加价格配置
- [ ] 添加预算配置

**预计时间**: 25 分钟

### Phase 10: 应用集成 ⏳ 待实现
- [ ] 更新 `src/app.rs` - 集成 ProviderManager
- [ ] 更新 `src/ui.rs` - 显示提供商信息
- [ ] 实现实时费用显示
- [ ] 实现速率限制显示

**预计时间**: 40 分钟

### Phase 11: 命令系统 ⏳ 待实现
- [ ] 更新 `src/commands.rs`
- [ ] 添加提供商命令
- [ ] 添加模型命令
- [ ] 添加使用量命令
- [ ] 添加预算命令

**预计时间**: 30 分钟

### Phase 12: 首次运行配置 ⏳ 待实现
- [ ] 创建 `src/setup.rs`
- [ ] 交互式提供商选择
- [ ] API Key 输入
- [ ] 模型选择
- [ ] 预算设置
- [ ] 连接测试

**预计时间**: 20 分钟

### Phase 13: 测试验证 ⏳ 待实现
- [ ] 单元测试各适配器
- [ ] 集成测试自动切换
- [ ] 测试速率限制
- [ ] 测试预算限制
- [ ] 测试使用量追踪

**预计时间**: 30 分钟

### Phase 14: 文档更新 ⏳ 待实现
- [ ] 更新 `FEATURES.md`
- [ ] 创建 `docs/ai_providers.md`
- [ ] 创建 `docs/pricing.md`
- [ ] 创建 `docs/budget.md`

**预计时间**: 25 分钟

## 总预估时间

| 阶段 | 时间 |
|------|------|
| 核心基础设施 | 45 分钟 |
| 计费系统 | 30 分钟 |
| 速率限制 | 25 分钟 |
| 使用量追踪 | 35 分钟 |
| 预算管理 | 25 分钟 |
| 加密工具 | 15 分钟 |
| 提供商适配器 | 90 分钟 |
| 提供商管理器 | 45 分钟 |
| 配置系统 | 25 分钟 |
| 应用集成 | 40 分钟 |
| 命令系统 | 30 分钟 |
| 首次配置 | 20 分钟 |
| 测试验证 | 30 分钟 |
| 文档更新 | 25 分钟 |
| **总计** | **~8.5 小时** |

## 验证标准

- [ ] 所有提供商适配器编译通过
- [ ] 速率限制正确工作
- [ ] Token 计费准确
- [ ] 预算限制生效
- [ ] 使用量追踪准确
- [ ] API Key 加密/解密正确
- [ ] 自动切换正常工作
- [ ] 命令系统完整
- [ ] UI 显示正确
- [ ] 所有测试通过

## 技术依赖

```toml
[dependencies]
# 现有依赖
ratatui = "0.28"
crossterm = "0.28"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
anyhow = "1"
dirs = "5"
chrono = { version = "0.4", features = ["serde"] }

# 新增依赖
base64 = "0.22"  # Base64 编码
async-trait = "0.1"  # 异步 trait
```

## 未来扩展

1. **更多提供商** - 持续添加新提供商
2. **i18n 支持** - 多语言界面
3. **高级分析** - 使用量分析和建议
4. **智能切换** - 基于成本和性能的智能切换
5. **批量请求** - 支持批量请求降低成本
