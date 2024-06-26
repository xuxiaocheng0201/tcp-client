# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2024-3-21

### Changed

* 重构代码，简化不必要的定义。
* 更新依赖。

## [0.1.1] - 2024-1-27

### Fixed

* 修复feature错误。

### Changed

* 更新依赖。
* 优化文档。
* 简化内部mutable_cipher代码

## [0.1.0] - 2024-1-26

### Added

* 支持发送超时。
* 统一错误类型。
* 优化文档。

### Changed

* 更新依赖。

## [0.0.6] - 2024-1-9

### Fixed

* 修复check_func中处理响应格式错误问题。
* 使用timeout，优化性能。

## [0.0.5] - 2024-1-9

### Changed

* 扩展ClientBase泛型。

## [0.0.4] - 2024-1-9

### Added

* 添加MutableCipher和ClientBase，简化收发消息逻辑。

## [0.0.3] - 2024-1-4

### Added

* 重命名为ClientFactory。
* 更新依赖。

## [0.0.2] - 2024-1-1

### Added

* 支持默认version。
* 支持快捷连接。

## [0.0.1] - 2024-1-1

### Added

* 实现连接Server。
* 支持序列化配置。
