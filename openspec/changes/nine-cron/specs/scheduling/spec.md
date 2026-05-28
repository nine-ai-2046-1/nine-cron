## ADDED Requirements

### Requirement: cron-based schedule support
系統 MUST 支援 cron 表達式作為重複性排程嘅語法。每一個排程項目應包含一個 cron 表達式，並且指向要執行嘅指令。

#### Scenario: Schedule with cron expression
- **WHEN** 新增一個包含有效 cron 表達式嘅排程
- **THEN** scheduler 會喺該 cron 模式指定嘅時間觸發對應嘅指令執行

### Requirement: delay-based one-off runs
系統 SHALL 支援以延遲時間（例如 "5m"、"2h"）指定嘅一次性執行。

#### Scenario: Delayed execution
- **WHEN** 用戶請求一個延遲 5 分鐘嘅一次性執行
- **THEN** 系統會喺大約 5 分鐘後執行該指令一次

### Requirement: retries and backoff
系統 SHALL 支援每個排程可配置嘅重試次數同 Backoff 策略。

#### Scenario: Retry on failure
- **WHEN** 指令返回非零退出碼且重試次數 > 0
- **THEN** 系統會根據配置嘅 backoff 策略重試執行，直到重試次數用完或者成功為止
