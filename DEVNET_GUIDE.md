# Using Whale Tracker on Devnet/Mainnet

## ⚠️ Important: Geyser Plugin Limitations

**Geyser plugins only work when you run your OWN validator.** The plugin hooks into the validator process to receive transactions.

On public devnet/mainnet, you can't load plugins into someone else's validator.

---

## Options for Devnet/Mainnet

### Option 1: RPC Polling (Recommended - Free/Cheap)

Convert the plugin logic to a standalone app that polls for transactions:

```bash
# Using Helius (free tier available)
# 1. Get API key at https://helius.dev
# 2. Poll for USDC transactions
```

**Pros:** Free tier available, no validator needed  
**Cons:** Not real-time (polling delay)

---

### Option 2: Geyser gRPC Streaming (Real-time)

Use a hosted Geyser endpoint:

| Provider | Cost | Link |
|----------|------|------|
| Triton | $99/mo | triton.one |
| Helius | $49+/mo | helius.dev |

**Pros:** Real-time, same logic as your plugin  
**Cons:** Paid service

---

### Option 3: Run Your Own Validator (Expensive)

Run a full validator on devnet/mainnet with your plugin.

**Cost:** $350-950/month (server + bandwidth)  
**Only for:** Production apps with revenue

---

## Quick Start: RPC Polling

### 1. Create standalone Rust binary

```bash
cargo new whale_tracker
cd whale_tracker
```

### 2. Add dependencies

```toml
[dependencies]
solana-client = "3.0"
solana-sdk = "3.0"
tokio = { version = "1", features = ["full"] }
```

### 3. Poll for transactions

```rust
use solana_client::rpc_client::RpcClient;

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com");
    
    loop {
        // Get recent signatures for USDC program
        // Check token balances
        // Alert on large transfers
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
```

---

## Testing on Devnet Without Plugin

You can test the **logic** without a plugin:

```bash
# 1. Watch devnet USDC address manually
solana logs EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v --url devnet

# 2. Or use Helius webhooks for alerts
# https://docs.helius.dev/webhooks
```

---

## Summary

| Method | Real-time | Cost | Difficulty |
|--------|-----------|------|------------|
| Local validator + plugin | ✅ | Free | Easy |
| RPC polling | ❌ | Free | Medium |
| Geyser gRPC (Triton/Helius) | ✅ | $49-99/mo | Medium |
| Own mainnet validator | ✅ | $350+/mo | Hard |

**For testing:** Use local validator (what you have now)  
**For production:** Use Helius gRPC or webhooks
