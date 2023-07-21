type cw20coin = {
  address: string
  amount: string
}

type share = {
  address: string
  amount: string
}

// Init Messages
type collector = {}

type distributor = {
  token: string
  distribution: Array<[string, string]>
}

type staking = {
  fee_collector: string
  deposit_denom: string
  reward_denom: string
  deposit_decimals: number
  reward_decimals: number
  tokens_per_interval: string
  token_code_id: number
  token_name: string
}

interface Config {
  protocol_address: string
  manager_address: string
  collector: collector
  distributor: distributor
  staking: staking
}
