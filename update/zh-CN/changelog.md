# v4.0.1 (2024-11-27)

* fix: 先进行fas再比较 ([346a422](https://github.com/shadow3aaa/fas-rs/commit/346a422))
* fix: 废弃OFFSET_MAP以修复offset和新策略不兼容的问题 ([ffaca27](https://github.com/shadow3aaa/fas-rs/commit/ffaca27))

# v4.0.0 (2024-11-27)

* refactor: 使用透明错误消息 ([9cc0b2a](https://github.com/shadow3aaa/fas-rs/commit/9cc0b2a))
* refactor: 打包README.md改为README_CN.md ([e9b4d7f](https://github.com/shadow3aaa/fas-rs/commit/e9b4d7f))
* refactor: 移除部分错误处理日志 ([ed5e0fd](https://github.com/shadow3aaa/fas-rs/commit/ed5e0fd))
* refactor: 调整 ControllerParams 的默认 kp 值 ([616ae45](https://github.com/shadow3aaa/fas-rs/commit/616ae45))
* build(deps): bump libc from 0.2.164 to 0.2.166 ([a27d7ba](https://github.com/shadow3aaa/fas-rs/commit/a27d7ba))
* build(deps): bump sysinfo from 0.32.0 to 0.32.1 ([8f81530](https://github.com/shadow3aaa/fas-rs/commit/8f81530))
* build(deps): 更新依赖 ([9b35785](https://github.com/shadow3aaa/fas-rs/commit/9b35785))
* Revert "feat: 添加trim-paths功能以优化构建" ([af8d3f9](https://github.com/shadow3aaa/fas-rs/commit/af8d3f9))
* feat: 使用负载跟踪辅助fas调频 ([0241394](https://github.com/shadow3aaa/fas-rs/commit/0241394))
* feat: 将自动微调目标fps的判断方法由fps方差改为占用率监测 ([a359a2f](https://github.com/shadow3aaa/fas-rs/commit/a359a2f))
* fix: 调整微调目标fps的占用率阈值 ([ca10192](https://github.com/shadow3aaa/fas-rs/commit/ca10192))
