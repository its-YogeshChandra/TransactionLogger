#!/bin/bash
# simple_local_test.sh - Test whale alerts on local validator

CLUSTER="--url http://127.0.0.1:8899"

echo "🐋 Whale Alert Test"
echo "==================="
echo ""

# Set config to use local validator
echo "Step 0: Configuring for local validator..."
solana config set --url http://127.0.0.1:8899 >/dev/null 2>&1
echo ""

# Airdrop
echo "Step 1: Getting SOL..."
solana airdrop 5 $CLUSTER 2>/dev/null || true
echo ""

# Create token
echo "Step 2: Creating token..."
spl-token create-token $CLUSTER --decimals 6
echo ""
read -p "Paste token address: " TOKEN

# Create account
echo "Step 3: Creating token account..."
spl-token create-account $TOKEN $CLUSTER
echo ""

# Mint
echo "Step 4: Minting tokens..."
spl-token mint $TOKEN 100000 $CLUSTER
echo ""

# Recipient
echo "Step 5: Creating recipient keypair..."
solana-keygen new --no-bip39-passphrase -o /tmp/recipient.json --force 2>/dev/null
RECIPIENT=$(solana-keygen pubkey /tmp/recipient.json)
echo "Recipient: $RECIPIENT"

echo "Funding recipient..."
solana transfer $RECIPIENT 0.5 $CLUSTER --allow-unfunded-recipient
echo ""

echo "Step 6: Creating recipient token account..."
spl-token create-account $TOKEN --owner $RECIPIENT --fee-payer ~/.config/solana/id.json $CLUSTER
echo ""
read -p "Paste recipient token account: " RECIPIENT_ACCOUNT

echo ""
echo "🎯 Sending transfers..."
echo ""

echo "Test 1: 500 tokens (no alert)"
spl-token transfer $TOKEN 500 $RECIPIENT_ACCOUNT $CLUSTER --fund-recipient
sleep 2

echo ""
echo "Test 2: 5000 tokens (WHALE!)"
spl-token transfer $TOKEN 5000 $RECIPIENT_ACCOUNT $CLUSTER --fund-recipient
sleep 2

echo ""
echo "Test 3: 2000 tokens (WHALE!)"
spl-token transfer $TOKEN 2000 $RECIPIENT_ACCOUNT $CLUSTER --fund-recipient
echo ""

echo "✅ Done! Check validator for 🐋 alerts"
echo "Or: cat whale_alerts.txt"
