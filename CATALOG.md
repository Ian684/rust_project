# 50 題練習目錄

題號代表獨立實作單元，不把測試案例、重複輸入或程式行數算成額外題目。原本的專案檔案仍保留，以下是這次有說明及驗證的 50 個單元。

## 執行與驗證

    cargo test --offline --manifest-path practice/Cargo.toml
    cargo test --offline --manifest-path practice/Cargo.toml exercise_42

## 題目

| 題號 | 任務／學習重點 | 來源 | 入口 |
|---|---|---|---|
| 01 | 泛型堆疊 | [lib.rs](practice/src/lib.rs) | Stack |
| 02 | 數值統計 | [lib.rs](practice/src/lib.rs) | parse_statistics |
| 03 | 借用單字索引 | [lib.rs](practice/src/lib.rs) | word_positions |
| 04 | 分段執行緒加總 | [lib.rs](practice/src/lib.rs) | parallel_sum |
| 05 | 移動 tuple 所有權 | [more.rs](practice/src/more.rs) | swap_pair |
| 06 | 相鄰重複值與 Vec | [more.rs](practice/src/more.rs) | dedup_adjacent |
| 07 | 切片分塊與錯誤 | [more.rs](practice/src/more.rs) | chunks |
| 08 | 迭代器攤平 | [more.rs](practice/src/more.rs) | flatten |
| 09 | entry 計數 | [more.rs](practice/src/more.rs) | frequencies |
| 10 | 依字元數分組 | [more.rs](practice/src/more.rs) | group_length |
| 11 | partition 拆分集合 | [more.rs](practice/src/more.rs) | partition_even |
| 12 | scan 前綴累積 | [more.rs](practice/src/more.rs) | running_totals |
| 13 | zip 的長度契約 | [more.rs](practice/src/more.rs) | dot_product |
| 14 | checked_mul 溢位 | [more.rs](practice/src/more.rs) | factorial |
| 15 | 迴圈狀態與 checked_add | [more.rs](practice/src/more.rs) | fibonacci |
| 16 | 除以零與 MIN/-1 | [more.rs](practice/src/more.rs) | safe_divide |
| 17 | 借用的 Option | [more.rs](practice/src/more.rs) | safe_get |
| 18 | 字元不是位元組 | [more.rs](practice/src/more.rs) | reverse_unicode |
| 19 | 回傳原字串借用 | [more.rs](practice/src/more.rs) | first_word |
| 20 | 字串配置與正規化 | [more.rs](practice/src/more.rs) | word_counts |
| 21 | char_indices 的 byte 位移 | [more.rs](practice/src/more.rs) | character_offsets |
| 22 | Option 表達不存在 | [more.rs](practice/src/more.rs) | trim_nonempty |
| 23 | split_once 與 Result | [more.rs](practice/src/more.rs) | parse_property |
| 24 | match 白名單 | [more.rs](practice/src/more.rs) | parse_switch |
| 25 | ? 傳遞解析錯誤 | [more.rs](practice/src/more.rs) | sum_numbers |
| 26 | 排序副本不改輸入 | [more.rs](practice/src/more.rs) | median |
| 27 | 連號壓縮與溢位邊界 | [more.rs](practice/src/more.rs) | compress_ranges |
| 28 | 雙 iter 的 merge | [more.rs](practice/src/more.rs) | merge_sorted |
| 29 | 二分搜尋最左位置 | [more.rs](practice/src/more.rs) | first_equal |
| 30 | Vec 旋轉與空集合 | [more.rs](practice/src/more.rs) | rotate_left |
| 31 | 括號配對 | [more.rs](practice/src/more.rs) | balanced |
| 32 | 帶溢位檢查的運算式 | [more.rs](practice/src/more.rs) | postfix |
| 33 | 可變狀態的 Euclid | [more.rs](practice/src/more.rs) | gcd |
| 34 | Vec<bool> 篩法 | [more.rs](practice/src/more.rs) | primes |
| 35 | 二項式係數與 checked_add | [more.rs](practice/src/more.rs) | pascal_row |
| 36 | 矩形檢查與配置 | [more.rs](practice/src/more.rs) | transpose |
| 37 | 邊界收縮的矩陣走訪 | [more.rs](practice/src/more.rs) | spiral |
| 38 | 巢狀迭代器與 move | [more.rs](practice/src/more.rs) | cartesian |
| 39 | 不重複位置組合 | [more.rs](practice/src/more.rs) | index_pairs |
| 40 | Unicode 連續段 | [more.rs](practice/src/more.rs) | run_length |
| 41 | 位元組格式化 | [more.rs](practice/src/more.rs) | hex_encode |
| 42 | chunks_exact 與非法 hex | [more.rs](practice/src/more.rs) | hex_decode |
| 43 | 浮點數有限值驗證 | [more.rs](practice/src/more.rs) | celsius |
| 44 | 區間的 Result<Option> | [more.rs](practice/src/more.rs) | overlap |
| 45 | 排序後 take | [more.rs](practice/src/more.rs) | smallest |
| 46 | BTreeSet 集合差 | [more.rs](practice/src/more.rs) | set_difference |
| 47 | 保留出現次數的交集 | [more.rs](practice/src/more.rs) | multiset_intersection |
| 48 | windows API 與錯誤 | [more.rs](practice/src/more.rs) | window_maxima |
| 49 | 候選人再驗票 | [more.rs](practice/src/more.rs) | majority |
| 50 | 泛型閉包參數 | [more.rs](practice/src/more.rs) | apply_twice |

每個新增單元的正常值與邊界案例可在 tests/、check.py 或 testbench 找到。先預測輸出，再執行，再修改輸入觀察；不是把通過單一範例視為正式部署或線上 AC。
