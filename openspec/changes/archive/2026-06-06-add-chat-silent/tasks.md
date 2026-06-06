## 1. CLI Argument Definition

- [x] 1.1 Add `--silent` (`-s`) flag to Chat command in `src/main.rs`
- [x] 1.2 Pass `silent` parameter to `chat::run_chat()` call

## 2. Function Signatures

- [x] 2.1 Add `silent: bool` parameter to `run_chat()` function
- [x] 2.2 Add `silent: bool` parameter to `run_single()` function
- [x] 2.3 Pass `silent` through call chain

## 3. Conditional Output

- [x] 3.1 Wrap `print_header()` call in `if !silent` block
- [x] 3.2 Wrap `print_user_msg()` call in `if !silent` block
- [x] 3.3 Wrap `print_command()` call in `if !silent` block
- [x] 3.4 Ensure success message in `execute_schedule_add()` always displays
- [x] 3.5 Ensure error messages in `execute_schedule_add()` always display

## 4. Verification

- [x] 4.1 Build with `cargo build --release` and verify no errors
- [x] 4.2 Test: `nine-cron chat --silent --yes --msg "test"` shows only success
- [x] 4.3 Test: `nine-cron chat --silent --msg "test"` still shows confirmation prompt
- [x] 4.4 Test: `nine-cron chat --yes --msg "test"` shows full output (no regression)
