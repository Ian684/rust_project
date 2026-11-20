# Rust 練習

原本各 Cargo 專案保留。新增 practice/ 是獨立 package，沒有第三方 crate，以四個小實作練習所有權、錯誤傳遞、借用與執行緒。需要 Rust 1.63 以上。

    cargo run --manifest-path practice/Cargo.toml
    cargo test --offline --manifest-path practice/Cargo.toml

| 練習 | API | 要理解的事 |
|---|---|---|
| 泛型堆疊 | Stack<T> | push 移動值、peek 借用、pop 回傳 Option |
| 數值統計 | parse_statistics | Result、錯誤欄位、i64 輸入與 i128 累計 |
| 單字位置索引 | word_positions | BTreeMap 與借用 &str |
| 分段加總 | parallel_sum | scoped threads、借用切片、不整除分塊 |

Stack<String> 的 pop 交還所有權，peek 只提供參考；仍持有該參考時，不能任意修改堆疊。

word_positions 的結果借用原始文字，原文必須活得比索引久。位置是分詞後第幾個詞，不是位元組位移。它依空白分詞，保留標點與大小寫。

parse_statistics 接受逗號或換行分隔整數，空欄位不是 0。輸入 1,,2 會指出第二欄錯誤。累加使用 i128，容納多個 i64 大數。

parallel_sum 不保證比迴圈快，小輸入通常會被建立執行緒的成本抵銷。測試對照循序加總，涵蓋空陣列、零工作數、工作數大於資料量與不均分塊。

CI 驗證新增 package；舊專案是學習紀錄，不宣稱全 repo 是同一個可建置 workspace。
