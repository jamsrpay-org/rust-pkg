currencies=(
  cardano
  arbitrum
  avalanche
  binance_smart_chain
  bitcoin
  dogecoin
  polkadot
  ethereum
  chainlink
  litecoin
  optimism
  polygon
  solana
  sui
  ton
  tron
  stellar
  ripple
)

for c in "${currencies[@]}"; do
    echo "Processing $c"
    cargo new crates/$c --lib
done
