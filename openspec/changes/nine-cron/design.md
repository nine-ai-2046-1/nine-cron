## Context

本設計會為新嘅 Rust CLI `nine-cron`（原名 cli-scheduler）提供實作方案，用於排程同執行其他命令。Proposal 已定義好主要動機同能力（scheduling、execution、logging）。呢份設計會講點樣以模組化、可測試嘅方式喺 repo 裡面建立 crate（建議位置仍為：`crates/cli-scheduler`）。

限制條件：
- 儘量減少 runtime 依賴，並採用維護良好嘅 crates。
- 支援跨平台：Linux、macOS 與 Windows（當可能時）。
- 配置檔以 TOML 為主，並支援 JSON 作交換格式。

## Goals / Non-Goals

**Goals:**

**Goals:**
- 提供一個小而可靠嘅 CLI，能排程同執行其他 CLI，支援重試與日誌記錄。
- 支援 cron 類型嘅重複排程同一次性延遲執行。
- 提供清晰、可測試嘅行為，包含單元測試同整合測試。

**Non-Goals:**

**Non-Goals:**
- 唔會做分散式排程或完整嘅 job queue（唔做叢集協調）。
- 唔會提供複雜嘅 UI；只提供 CLI 同配置檔。

## Decisions

1. 語言與專案結構
   - 使用 Rust，建立 crate：`crates/cli-scheduler`，binary 名稱 `nine-cron`（原名 cli-scheduler）。
   - 理由：repo 內工具以 Rust 為主，Rust 提供可靠嘅 process control 與低運行時成本。

2. 排程解析
   - 採用 cron-like crate（例如 `cron` 或 `cron-parser`）處理 cron 表達式；一次性延遲以 duration 解析處理。
   - 理由：cron 為用戶熟悉嘅排程語法；將解析邏輯封裝，方便未來替換。

3. 進程執行
   - 使用 `std::process::Command` 處理進程啟動，並注意 stdout/stderr 流、退出碼、timeout 管理。
   - 提供 `Runner` trait 以便測試：實作 `SystemRunner`（實際執行）同 `MockRunner`（測試用）。

4. 配置
   - 預設使用 TOML 存放持久化排程（例如：`~/.config/nine-cron/schedules.toml`），CLI 亦可接受 JSON 輸入。支援從舊的 `~/.config/cli-scheduler/` 自動遷移。

5. 日誌
   - 使用 `tracing` 作為結構化日誌方案，並用 `tracing-appender` 或類似方案做檔案輪替；提供 per-schedule 日誌設定。

## Risks / Trade-offs


- Cron 相關 crate 喺不同平台上嘅品質差異：透過 adapter 隔離解析邏輯並寫單元測試以降低風險。
- 跨平台路徑同 process 差異：以 CI 上嘅整合測試（Linux/macOS）驗證；文件寫清楚平台差異與限制。

## Open Questions

- Should scheduled jobs run as background service/daemon or rely on the OS scheduler (systemd, launchd)? Initial scope: foreground binary with optional long-running mode. Defer OS service integration to a follow-up.
