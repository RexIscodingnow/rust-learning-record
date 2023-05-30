# rust 程式語言，學習筆記與紀錄

創建 rust 專案  ==>  Cargo

1. 創建程式碼 (或叫代碼)、下載依賴的程式庫
2. 命令行

    2-1. 版本查詢 => cargo --version 

    2-2. 專案新增 => cargo new 專案 (資料夾)
    
    2-3. 構建 (編譯) Cargo 項目 => cargo build
    
            2-3-1. 創建可執行檔案 => 目標路徑: ./target/debug/檔名.exe  (Windows)
    
            2-3-2. 執行編譯執行檔 => 目標路徑: ./target/debug/檔名.exe  (Windows)

    2-4. 構建 (編譯) 和 執行 => cargo run

    2-5. 第一次編譯，會生成 Cargo.lock 的檔案

    2-6. 檢查程式碼 (確認能通過編譯) => cargo check
    
            2-6-1. 用來確認是否可編譯通過，"但不會生成 執行檔案"
    
    2-7. 為發布構建 => cargo build --release
    
            2-7-1. 編譯時，會進行優化。執行速度會更快，但是編譯時間更長
    
            2-7-2. 生成的路徑會在 target/release 底下，而不是 target/debug 生成
    
            2-7-3. 編譯配置有兩種
                        1. 開發中
                        2. 正式發布 (該命令，屬於這類型)

# 之後詳見 rust_學習紀錄.txt 檔案