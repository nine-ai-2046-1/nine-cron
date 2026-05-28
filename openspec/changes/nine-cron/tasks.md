## 1. Project Setup

- [ ] 1.1 建立 Rust crate：`crates/cli-scheduler`，binary 名稱 `nine-cron`（原名 cli-scheduler）
- [ ] 1.2 加入依賴：`clap`、`tokio`（若需非同步）、`cron` 解析器、`tracing`、`tracing-appender`、`serde`、`serde_json`、`toml`
- [ ] 1.3 將 crate 加入 workspace（更新根目錄 Cargo.toml）

## 2. Core Execution Engine

- [ ] 2.1 實作 `Runner` trait，同埋用 `std::process::Command` 實作 `SystemRunner`
- [ ] 2.2 捕捉 stdout/stderr 並回傳退出狀態
- [ ] 2.3 支援喺臨時隔離工作目錄執行選項

## 3. Scheduling

- [ ] 3.1 實作排程解析（cron 表達式 + 延遲型一次性執行）
- [ ] 3.2 實作 scheduler loop，喺正確時間觸發執行
- [ ] 3.3 實作可配置嘅重試機制同 backoff

## 4. Logging & Persistence

- [ ] 4.1 結合 `tracing` 做結構化日誌
- [ ] 4.2 實作 per-run 日誌捕捉同檔案儲存
- [ ] 4.3 使用 `tracing-appender` 或相似方案做日誌輪替
- [ ] 4.4 實作 TOML 配置檔（讀/寫）並存喺 `~/.config/nine-cron/`（支援從舊位置 `~/.config/cli-scheduler/` 自動遷移）

## 5. CLI UX & Config

- [ ] 5.1 實作 CLI 指令：`run`、`schedule add`、`schedule list`、`schedule remove`、`daemon`（選項）
- [ ] 5.2 加入範例同說明（man/help）

## 6. Testing

- [ ] 6.1 單元測試：解析器同 Runner 抽象（使用 MockRunner）
- [ ] 6.2 整合測試：端到端執行（短命令）並喺 CI 上執行
- [ ] 6.3 CI 做跨平台驗證（至少 Linux + macOS）

## 7. Documentation

- [ ] 7.1 撰寫 README，包含使用範例
- [ ] 7.2 撰寫貢獻指南同未來 TODO
