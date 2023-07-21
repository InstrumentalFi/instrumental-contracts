export const testnet: Config = {
  protocol_address: 'osmo1rrmlcs4nr52uy239ljthnkhl9cvgfzvwdsjlch',
  manager_address: 'osmo164sw03784vjpqtf6ef8zs09zznrf7rnusw8v0e',
  collector: {},
  distributor: {
    token: 'uosmo',
    distribution: [
      ['osmo1rrmlcs4nr52uy239ljthnkhl9cvgfzvwdsjlch', '200000'],
      ['osmo164sw03784vjpqtf6ef8zs09zznrf7rnusw8v0e', '300000'],
    ],
  },
  staking: {
    fee_collector: '',
    deposit_denom: 'uosmo',
    reward_denom: 'uosmo',
    deposit_decimals: 6,
    reward_decimals: 6,
    tokens_per_interval: '0',
    token_code_id: 0,
    token_name: 'stakedOSMO',
  },
}
