# pet-rs WASM 插件系统开发计划

## 项目概述

pet-rs 是一个基于 Bevy ECS 的虚拟宠物框架，支持 WASM 插件系统。

## 当前状态

### ✅ 已完成的功能

#### 1. 核心框架（100%）
- WASM 运行时（wasmtime v24）
- 热重载支持
- 插件状态存储
- 插件间通信

#### 2. 生命周期钩子（100%）
- `on_load` - 插件加载时调用
- `on_unload` - 插件卸载时调用
- `on_error` - 错误处理钩子

#### 3. 配置系统（100%）
- JSON 配置文件支持
- 配置读取接口
- 配置解析和验证

#### 4. 依赖管理（100%）
- 依赖声明格式
- 拓扑排序算法
- 循环依赖检测
- 自动依赖加载

#### 5. 版本检查（100%）
- 语义化版本解析
- 版本比较和验证
- 版本要求检查（>=, <=, >, <, ^, ~）

#### 6. 权限控制（100%）
- 权限模型定义
- 权限检查接口
- 配置文件权限声明
- 拒绝权限支持

## 当前任务

### 🎯 目标
改进 basic_pet.rs 示例，清晰展示所有新功能。

### 📋 具体任务

#### 1. 增强 UI 显示
- [ ] 显示每个插件的版本号
- [ ] 显示权限检查状态
- [ ] 显示依赖关系
- [ ] 显示插件加载状态

#### 2. 添加交互功能
- [ ] 按 R 键触发热重载
- [ ] 按 I 键显示插件详细信息
- [ ] 按 P 键测试权限控制

#### 3. 改进日志输出
- [ ] 在生命周期钩子中输出详细日志
- [ ] 在权限检查时输出警告
- [ ] 在依赖解析时输出加载顺序

#### 4. 添加新的 UI 系统
- [ ] 创建插件信息面板
- [ ] 显示实时权限检查结果
- [ ] 显示依赖图

### 🔧 实现细节

#### 改进 1：增强 UI 显示

在 `update_ui` 函数中添加：
```rust
// 显示插件版本信息
let version_text = format!("  Plugin Versions:");
// - demo_plugin: 1.0.0
// - stats_plugin: 1.0.0
// - reader_plugin: 1.0.0

// 显示权限状态
let permission_text = format!("  Permissions:");
// - demo_plugin: FULL ACCESS
// - stats_plugin: READ/WRITE DATA
// - reader_plugin: READ DATA (DENIED WRITE)

// 显示依赖关系
let dependency_text = format!("  Dependencies:");
// - demo_plugin: none
// - stats_plugin: -> demo_plugin
// - reader_plugin: -> stats_plugin
```

#### 改进 2：添加交互系统

添加新的按键处理：
```rust
// 按 R 键热重载插件
if keys.just_pressed(KeyCode::KeyR) {
    info!("Hot reloading plugins...");
    // 重新加载配置
    // 重新加载插件
}

// 按 I 键显示插件信息
if keys.just_pressed(KeyCode::KeyI) {
    info!("Plugin Information:");
    // 显示每个插件的详细信息
}

// 按 P 键测试权限
if keys.just_pressed(KeyCode::KeyP) {
    info!("Testing permissions...");
    // 测试读取权限
    // 测试写入权限
    // 显示拒绝日志
}
```

#### 改进 3：增强 setup_ui

在 `setup_ui` 函数中添加详细的加载日志：
```rust
info!("=== Plugin Loading Order (Dependencies) ===");
info!("1. demo_plugin (no dependencies)");
info!("2. stats_plugin (depends on demo_plugin)");
info!("3. reader_plugin (depends on stats_plugin)");

info!("=== Plugin Versions ===");
info!("- demo_plugin: v1.0.0");
info!("- stats_plugin: v1.0.0");
info!("- reader_plugin: v1.0.0");

info!("=== Plugin Permissions ===");
info!("- demo_plugin: FULL ACCESS");
info!("- stats_plugin: READ/WRITE DATA, READ CONFIG");
info!("- reader_plugin: READ DATA ONLY");
```

### 📊 测试计划

1. **基本功能测试**
   - 验证插件加载正常
   - 验证 UI 显示正确
   - 验证按键交互响应

2. **功能展示测试**
   - 验证版本号显示
   - 验证权限状态显示
   - 验证依赖关系显示

3. **交互功能测试**
   - 验证热重载功能
   - 验证插件信息显示
   - 验证权限测试功能

### 🎯 预期效果

用户运行示例后可以看到：
1. 清晰的插件加载顺序（依赖管理）
2. 每个插件的版本号（版本检查）
3. 权限配置和检查状态（权限控制）
4. 配置文件加载过程（配置系统）
5. 生命周期钩子日志（生命周期钩子）

### 📝 下一步行动

1. 修改 `update_ui` 函数添加版本和权限显示
2. 添加新的按键交互系统
3. 改进 `setup_ui` 显示详细加载信息
4. 添加插件信息面板系统
5. 测试验证所有功能
