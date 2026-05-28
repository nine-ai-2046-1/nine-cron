## ADDED Requirements

### Requirement: stdout/stderr capture
系統 SHALL 捕捉每次執行嘅 stdout 同 stderr，並將佢哋存放喺與該次執行關聯嘅日誌檔案內。

#### Scenario: Capture logs
- **WHEN** 指令喺 stdout 同 stderr 輸出內容
- **THEN** 該輸出會分別記錄喺與該次執行相關嘅日誌檔案內

### Requirement: log rotation
系統 SHALL 根據檔案大小或年齡進行日誌輪替，該行為應該可由用戶配置。

#### Scenario: Rotate logs
- **WHEN** 日誌檔案超出配置嘅最大大小
- **THEN** 系統會輪替該日誌並保留配置嘅歷史檔案數量

### Requirement: structured logging
系統 SHOULD 輸出結構化日誌（JSON）以包含執行元資料（時間戳、退出碼、持續時間）。

#### Scenario: Structured metadata
- **WHEN** 一次執行完成
- **THEN** 系統會輸出一行 JSON，內含 run id、開始/結束時間、退出碼、執行時長
