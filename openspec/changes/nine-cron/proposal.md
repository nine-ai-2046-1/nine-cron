## Why

排程同編排其他 CLI 喺開發同自動化流程入面好常見。做一個用 Rust 寫嘅小而專注嘅 CLI（可以排程同執行其他命令，支援重試、記錄、隔離工作目錄，並可選擇時間排程）可以減少專案之間嘅重複腳本，提供一個安全、可測試、同跨平台嘅方法去執行排程任務。

## What Changes

- 新增一個用 Rust 開發嘅 CLI：`nine-cron`（之前名為 cli-scheduler）。主要功能包括：
  - 執行其他 CLI 命令，並支援傳參數同環境變數
  - 支援一次性執行、延遲執行，以及基於時間嘅排程（類 cron）
  - 支援重試策略、backoff 同退出碼處理
  - 捕捉 stdout/stderr 到日誌檔案，並支援日誌輪替
  - 可選喺臨時隔離工作目錄執行命令
  - 提供簡單嘅配置格式（TOML / JSON）用作持久化排程
- 新增 specs：`scheduling`、`execution`、`logging`
- 提供 design.md 同 tasks.md，支援實作此二進位檔同基本整合測試

## Capabilities

### New Capabilities
- `scheduling`: Defines scheduling semantics (cron expressions, delays, retries)
- `execution`: Defines how to spawn and manage child processes, envs, and working dirs
- `logging`: Defines logging format, rotation, and storage locations

### Modified Capabilities
- 

## Impact

- 會新增一個 Rust 二進位檔，建議位置：`crates/cli-scheduler`（binary 名稱：`nine-cron`，原為 cli-scheduler）
- 會新增用戶端配置檔，預設放喺 `~/.config/nine-cron/`（支援從舊的 `~/.config/cli-scheduler/` 自動遷移），並可支援專案內置配置
- 可能會加入少量 runtime 依賴（例如 cron 解析、process 管理、及 logging crates），需審核相依性品質
