## 目錄結構

```text
.
├── src/           
│   ├── models/          # 統一各種model
│   └── repo/            # 操作Database的功能(由service傳遞)
│   └── router/          # 統一管理router
│   └── service/         # 各種功能(不操作Database)
│    ── main.rs          # 入口文件
│    ── lib.rs           # module 引入
│    ── error.rs         # 自定義錯誤
│    ── state.rs         # 通用狀態
```