# nine-cron 使用說明 (廣東話)

nine-cron 係一個輕量嘅 CLI 排程工具，用嚟喺本地排期執行其他 command、用 NLJSON 逐行輸出 stdout/stderr，同埋管理一次性或重複排程。

功能簡介
- 即時執行命令並以 NLJSON 逐行輸出
- 支援相對時間(+6s, +1h30m)同埋重複 token (1d, 3w, 4mo)
- daemon 模式自動觸發到期任務
- 排程會以 TOML 存放喺系統 config 目錄
- 排程 ID 用英文字母同數字（冇連字元），方便複製貼上

何時使用 (Use cases)

- 當 agent 需要喺特定時間提醒用戶時，可以用 nine-cron 來註冊提醒命令。例如 agent 要發送提醒，可呼叫：

```bash
./target/release/nine-cron schedule add -T "提醒收租" +1h -- "opencb send 'this is msg to user for reminding sth'"
```

如果用戶表明呢個提醒係會重覆發生（例如每日、每周），就加 `-r`（例如 `-r "1d"`）。如果用戶冇提但 agent 判斷應該係重覆，agent 應該先問用戶要唔要設定重覆，然後先用 nine-cron 去建立排程。

如果你想知道現時有啲咩 scheduled task，就用：

```bash
./target/release/nine-cron schedule list
```
會以人類易讀嘅表格格式顯示。

快速示例

- 即時執行：

```bash
./target/release/nine-cron run "echo 你好"
```

 - 增加 6 秒後執行嘅一次性排程（如果命令包含空格或 shell 特殊字元請用引號包住）:

```bash
./target/release/nine-cron schedule add +6s "echo 早晨"
```

- 每日 12:00 執行（重複）：

```bash
./target/release/nine-cron schedule add -t 12:00 -r "1d" echo daily-job
```

 - 啟動 daemon（預設 poll interval 10s，使用 -i 改變秒數）:

```bash
./target/release/nine-cron daemon -i 10
```

 - 刪除排程（用 ID）:

```bash
./target/release/nine-cron schedule remove <id>
```

 - 刪除所有排程（需確認）:

```bash
./target/release/nine-cron schedule remove --all
```

 - 刪除所有排程（免確認）:

```bash
./target/release/nine-cron schedule remove --all -y
```

AI 對話

用自然語言建立排程，支援 AI 智能對話：

```bash
# 單次訊息
./target/release/nine-cron chat --title "任務名稱" --msg "提醒我每週二朝早9點打電話畀媽咪"

# 互動模式（多輪對話）
./target/release/nine-cron chat --title "任務名稱" --interactive

# 自動確認（唔使問）
./target/release/nine-cron chat --title "任務名稱" --msg "每晚2點跑backup" -y
```

AI 對話功能：
- 用 AI 理解自然語言，自動建立排程
- 支援相對時間：「2個鐘後提醒我」、「3日後提醒我」
- 自動生成 `opencb send` 提醒訊息
- 互動模式支援多輪對話
- 意圖模板存放喺 `~/.config/nine-cron/chats/intentions/`

Daemon 行為同 logs

- daemon 會周期性檢查並執行到期嘅排程；daemon 本身唔會不停喺 stdout 打 detailed logs，避免 background 執行時干擾。
- 每次執行之 metadata / log 會儲存在 data 目錄底下嘅 `runs/`（例如 Linux: `~/.local/share/nine-cron/runs/<run_id>.log`）。
- 當你新增 schedule（`nine-cron schedule add`）成功時，CLI 會輸出 `schedule added`。

檔案位置
- 配置檔：系統 config 目錄底下，project 名稱為 `nine-cron`
- 每次執行嘅 log 存放喺 data dir 下嘅 `runs/` 資料夾
