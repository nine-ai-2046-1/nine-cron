## ADDED Requirements

### Requirement: spawn external commands
系統 SHALL 使用安全嘅 child process API 去 spawn 外部指令，並按配置傳遞參數同環境變數。

#### Scenario: Execute command with args
- **WHEN** 排程指定指令 `echo` 同參數 `["hello"]`
- **THEN** 系統會執行 `echo hello`，並捕捉 stdout/stderr

### Requirement: isolated working directory
系統 SHALL 支援可選嘅臨時隔離工作目錄，當排程要求時啟用。

#### Scenario: Isolated run
- **WHEN** 排程要求使用隔離工作目錄
- **THEN** 指令會喺臨時目錄內執行，成功後清除該目錄

### Requirement: exit status handling
系統 SHALL 呈現執行嘅退出狀態，並相應地標記該次執行為成功或失敗。

#### Scenario: Non-zero exit
- **WHEN** 指令以狀態碼 2 結束
- **THEN** 該次執行會被記錄為失敗，並按照排程策略嘗試重試
