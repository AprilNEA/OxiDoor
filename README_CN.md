<h4 align="right"><a href="./README_EN.md">English</a> | <strong>简体中文</strong></h4>

>[!WARNING]
> 🚧开发中，尚未准备好生产。

>[!NOTE]
> [Unifi Protect](https://ui.com/us/en/physical-security/door-access) 太贵了，所以为什么不自己做呢?

# OXIDOOR 🔒

基于 Rust 和 ESP32-S3 构建的智能门锁系统，支持多种连接方式和供电方法。

## 功能特性

- 🔐 **智能门锁控制**: 基于 GPIO 继电器开关的电磁锁控制
- 🎮 **多输入支持**: 双按钮输入，支持手动操作
- 🔗 **连接就绪**: 为 NFC、蓝牙、Wi-Fi 和以太网集成而设计
- ⚡ **电源选项**: 支持 PoE（以太网供电）供电
- 🦀 **Rust 驱动**: 基于 Rust 构建，确保 ESP32-S3 上的安全性和性能
- ⚡ **异步运行时**: 基于 Embassy 的异步任务处理

## 硬件要求

- ESP32-S3 微控制器
- 电磁门锁（通过 GPIO5 控制）
- 两个输入按钮（GPIO6, GPIO7）
- 电源供应（标准或 PoE）
- PN532/PN5180 NFC 读卡器
- 3.3V 继电器

## 当前实现

当前版本提供基本的门锁功能：

- **门锁控制**: 连接到 GPIO5 的电磁锁
- **按钮输入**: 两个按钮（GPIO6, GPIO7），带内部上拉电阻
- **自动上锁**: 按下按钮后 3 秒解锁时长
- **异步操作**: 使用 Embassy 执行器的非阻塞操作

## 构建和烧录

### 前置要求

```bash
# 安装 Rust ESP 工具链
cargo install espup
espup install

# 加载环境变量
source ~/export-esp.sh
```

### 构建和烧录

```bash
# 构建项目
cargo build

# 烧录到 ESP32-S3
cargo run
```

## 引脚配置

| 功能 | GPIO 引脚 | 说明 |
|------|-----------|------|
| 门锁 | GPIO5 | 继电器控制输出（高电平 = 锁定，低电平 = 解锁） |
| 按钮 1 | GPIO6 | 带上拉输入（按下时为低电平） |
| 按钮 2 | GPIO7 | 带上拉输入（按下时为低电平） |

## 项目结构

```
src/
├── main.rs          # 主应用程序入口点
└── task.rs          # 异步门锁控制任务实现
```

## 未来路线图

- 📱 NFC 卡片认证
- 📡 蓝牙低功耗支持
- 🌐 Wi-Fi 连接和远程控制
- 🔌 以太网连接
- 📊 访问日志和监控
- 🔒 增强安全功能
- 📱 移动应用集成

## 许可证

基于 Apache License, Version 2.0 许可。详情请参阅 [LICENSE](LICENSE) 文件。

## 作者

- [AprilNEA](https://github.com/aprilnea)

仓库地址：[https://github.com/aprilnea/oxidoor](https://github.com/aprilnea/oxidoor)

---

*OXIDOOR - 用 Rust 的可靠性保护您的空间*