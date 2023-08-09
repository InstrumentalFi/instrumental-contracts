export const testnet: Config = {
  protocol_address: 'osmo1rrmlcs4nr52uy239ljthnkhl9cvgfzvwdsjlch',
  manager_address: 'osmo164sw03784vjpqtf6ef8zs09zznrf7rnusw8v0e',
  collector: {},
  distributor: {
    token:
      'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477',
    distribution: [
      ['osmo1rrmlcs4nr52uy239ljthnkhl9cvgfzvwdsjlch', '200000'],
      ['osmo164sw03784vjpqtf6ef8zs09zznrf7rnusw8v0e', '300000'],
    ],
  },
  staking: {
    fee_collector: '',
    deposit_denom: 'uosmo',
    reward_denom:
      'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477',
    deposit_decimals: 6,
    reward_decimals: 6,
    tokens_per_interval: '10000',
    token_code_id: 0,
    token_name: 'stakedOSMO',
  },
}
