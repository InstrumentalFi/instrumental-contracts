#!/usr/bin/env sh
#

COLLECTOR_CONTRACT_NAME=../artifacts/collector.wasm
STAKING_CONTRACT_NAME=../artifacts/staking.wasm
NODE_ADDRESS=127.0.0.1:9988

# Check ccw is installed
if ! command -v ccw >/dev/null; then
  echo "ccw command not found" >&2
  exit 1
fi

# Store the Collector contract
printf '%s\n' '--- STORE COLLECTOR CONTRACT ---'
COLLECTOR_STORE_CODE_OUTPUT=$(
  ccw substrate --node ws://$NODE_ADDRESS \
    --from alice --output json \
    tx store ./$COLLECTOR_CONTRACT_NAME
)

printf '\n%s\n' '--- STORE COLLECTOR CODE RESPONSE ---'
echo "$COLLECTOR_STORE_CODE_OUTPUT"

COLLECTOR_CODE_ID=$(echo "$COLLECTOR_STORE_CODE_OUTPUT" | jq '.extrinsic.details.code_id')

echo $COLLECTOR_CODE_ID

# instantiate the collector contract
printf '\n%s\n' '--- instantiate collector contract ---'
COLLECTOR_INSTANTIATE_OUTPUT=$(
  ccw substrate --node ws://$NODE_ADDRESS \
    --from alice --output json tx instantiate2 "$COLLECTOR_CODE_ID" "{}" \
    0x9999 --label 0x1111 --gas 100000000000
)

printf '\n%s\n' '--- instantiate collector contract response ---'
echo "$COLLECTOR_INSTANTIATE_OUTPUT"

COLLECTOR_CONTRACT_ADDRESS=$(echo "$COLLECTOR_INSTANTIATE_OUTPUT" | jq '.cosmwasm_events[0].contract' -r)

# Query collector config
printf '\n%s\n' '--- QUERY COLLECTOR CONFIG ---'
COLLECTOR_CONFIG=$(ccw substrate --node http://$NODE_ADDRESS \
  --output json query wasm --contract "$COLLECTOR_CONTRACT_ADDRESS" \
  --gas 10000000000 \
  --query '{"config": {}}'
)
printf '\n%s\n' '--- query collector config response ---'
echo "$COLLECTOR_CONFIG"

# Query collector owner
printf '\n%s\n' '--- QUERY COLLECTOR OWNER ---'
COLLECTOR_OWNER=$(ccw substrate --node http://$NODE_ADDRESS \
  --output json query wasm --contract "$COLLECTOR_CONTRACT_ADDRESS" \
  --gas 10000000000 \
  --query '{"get_owner": {}}'
)
printf '\n%s\n' '--- query collector owner response ---'
echo "$COLLECTOR_OWNER"

# Add a token to the collector
printf '\n%s\n' '--- ADD TOKEN TO COLLECTOR ---'
COLLECTOR_ADD_TOKEN=$(ccw substrate --node ws://$NODE_ADDRESS \
  --output json tx execute --contract "$COLLECTOR_CONTRACT_ADDRESS" \
  --gas 10000000000 \
  --message '{"add_token": {"token": "1"}}'
)
printf '\n%s\n' '--- query collector owner response ---'
echo "$COLLECTOR_ADD_TOKEN"

# Query collector token
printf '\n%s\n' '--- QUERY COLLECTOR OWNER ---'
COLLECTOR_OWNER=$(ccw substrate --node http://$NODE_ADDRESS \
  --output json query wasm --contract "$COLLECTOR_CONTRACT_ADDRESS" \
  --gas 10000000000 \
  --query '{"is_token": {"token": "1"}}'
)
printf '\n%s\n' '--- query collector owner response ---'
echo "$COLLECTOR_OWNER"

# Store the Staking contract
printf '%s\n' '--- STORE STAKING CONTRACT ---'
STAKING_STORE_CODE_OUTPUT=$(
  ccw substrate --node ws://$NODE_ADDRESS \
    --from alice --output json \
    tx store ./$STAKING_CONTRACT_NAME
)

printf '\n%s\n' '--- STORE STAKING CODE RESPONSE ---'
echo "$STAKING_STORE_CODE_OUTPUT"


STAKING_CODE_ID=$(echo "$STAKING_STORE_CODE_OUTPUT" | jq '.extrinsic.details.code_id')

echo $STAKING_CODE_ID

# instantiate the staking contract
printf '\n%s\n' '--- instantiate staking contract ---'
STAKING_INSTANTIATE_OUTPUT=$(
  ccw substrate --node ws://$NODE_ADDRESS \
    --from alice --output json tx instantiate2 "$STAKING_CODE_ID" \
    "{\"fee_collector\": \"$COLLECTOR_CONTRACT_ADDRESS\", 
      \"deposit_denom\": \"1\"
      \"reward_denom\": \"1\"
      \"deposit_decimals\": 18
      \"reward_decimals\": 18
      \"tokens_per_interval\": 1000000000
    }" \
    --label 0x1112 --gas 100000000000 0x9902
)

# Query staking config
printf '\n%s\n' '--- QUERY STAKING CONFIG ---'
STAKING_CONFIG=$(ccw substrate --node http://$NODE_ADDRESS \
  --output json query wasm --contract "$STAKING_CONTRACT_ADDRESS" \
  --gas 10000000000 \
  --query '{"config": {}}'
)
printf '\n%s\n' '--- query collector owner response ---'
echo "$STAKING_CONFIG"

# Query staking config
printf '\n%s\n' '--- QUERY STAKING STATE ---'
STAKING_STATE=$(ccw substrate --node http://$NODE_ADDRESS \
  --output json query wasm --contract "$STAKING_CONTRACT_ADDRESS" \
  --gas 10000000000 \
  --query '{"state": {}}'
)
printf '\n%s\n' '--- query collector owner response ---'
echo "$STAKING_STATE"

## Mint some stake tokens for Alice
#printf '\n%s\n' '--- MINTING TOKENS FOR ALICE ---'
#ccw substrate --node ws://$NODE_ADDRESS \
#  --from alice --output json tx execute --contract \
#  "$CW20_CONTRACT_ADDRESS" --gas 10000000000 --message \
#  '{ "mint" : { "amount" : "123456789", "recipient" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" }}'
#
## Query Alice's balance
#printf '\n%s\n' '--- QUERY ALICE BALANCE ---'
#ccw substrate --node http://$NODE_ADDRESS \
#  --output json rpc query --contract "$CW20_CONTRACT_ADDRESS" \
#  --gas 10000000000 \
#  --query '{"balance": {"address": "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"}}'
#
## Store the CW4 contract
#printf '%s\n' '--- STORE CW4 CONTRACT ---'
#CW4_STORE_CODE_OUTPUT=$(
#  ccw substrate --node ws://$NODE_ADDRESS \
#    --from alice --output json \
#    tx store ./$CW4_CONTRACT_NAME
#)
#printf '\n%s\n' '--- STORE CW4 CODE RESPONSE ---'
#echo "$CW4_STORE_CODE_OUTPUT"
#
#CW4_CODE_ID=$(echo "$CW4_STORE_CODE_OUTPUT" | jq '.extrinsic.details.code_id')
#
## Instantiate the CW4 contract
#printf '\n%s\n' '--- INSTANTIATE CW4 CONTRACT ---'
#CW4_INSTANTIATE_OUTPUT=$(
#  ccw substrate --node ws://$NODE_ADDRESS \
#    --from alice --output json tx instantiate "$CW4_CODE_ID" \
#    '{ "denom" : "'"$CW20_CONTRACT_ADDRESS"'", "tokens_per_weight": 1000, "min_bond": 5000, "unbonding_period": 10000 }' \
#    --salt 0x9991 --label 0x1112 --gas 10000000000
#)
#printf '\n%s\n' '--- INSTANTIATE CW4 CONTRACT RESPONSE ---'
#echo "$CW4_INSTANTIATE_OUTPUT"

exit 0
