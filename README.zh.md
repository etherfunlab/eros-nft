# eros-nft

> Solana cNFT 上预定义 AI 人格卡的开放标准。

人格卡是一个独立的 Solana cNFT，持有者凭它获得与某个预定义 AI 人格对话的权限。
本仓库定义格式、发布 JSON Schema 2020-12 契约、提供 Rust 参考实现 crate
（crates.io 上的 `eros-nft`）、并附带 15 个示例人格。

[English](README.md) · 中文

## 仓库内容

| 路径 | 说明 |
|---|---|
| `spec/v1.0/` | 规范文档（CC-BY-4.0）。 |
| `spec/v1.0/schemas/` | `persona-draft.schema.json`、`persona-manifest.schema.json`（JSON Schema 2020-12，Apache-2.0）。 |
| `crates/eros-nft/` | Rust 参考实现 crate（Apache-2.0）。类型、校验器、sample loader、CLI。 |
| `samples/` | 15 个示例人格（5 个 NSFW）。每个含 `draft.json` + `manifest.json` + `README.md`。 |

## 快速开始

### 用 CLI 校验文档

```bash
cargo install eros-nft
eros-nft validate ./my-persona-manifest.json
```

### 在 Rust 项目中使用

```toml
[dependencies]
eros-nft = "0.1"
```

```rust
use eros_nft::load_sample;

fn main() {
    let (_draft, manifest) = load_sample("yuki-warm-senpai").unwrap();
    manifest.validate().unwrap();
    println!("{} ({})", manifest.name, manifest.persona_id);
}
```

## 设计要点

- **`PersonaDraft`**：创作者提交给市集铸造流水线的输入。含明文 prompt + 原始头像，仅在流水线内存中存在。
- **`PersonaManifest`**：发布产物。适合钉到 Arweave / IPFS 或作为 Solana cNFT 的 metadata URI。只携带 `prompt_ciphertext_ref`（KMS 引用 + 密文 SHA-256），不含明文 prompt。
- cNFT 本身是链上锚定，Manifest 与链无关。详见 [`spec/v1.0/06-chain-profiles/solana-cnft.md`](spec/v1.0/06-chain-profiles/solana-cnft.md)。

## 不在范围内

- 训练后人格转手（dossier、lineage、训练指标）由未来的 `eros-nft-extended` 规范定义。
- 市集商业逻辑（铸造流水线、royalty 强制、takedown）由 `eros-chat-marketplace`（闭源）实现。

## License

- 规范文档：**CC-BY-4.0**（见 [LICENSE-SPEC](LICENSE-SPEC)）
- 代码和 JSON Schema：**Apache-2.0**（见 [LICENSE-CODE](LICENSE-CODE)）
