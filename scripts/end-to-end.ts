import 'dotenv/config.js'
import {
  deployContract,
  executeContract,
  queryContract,
  uploadContract,
  queryBalance,
  sendToken,
} from './helpers.js'
import { GasPrice, setupNodeLocal, Coin } from 'cosmwasm'
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing'
import { toUtf8 } from '@cosmjs/encoding'
import { writeFile } from 'fs'
import { testnet } from './deploy_configs.js'
import { join } from 'path'
import readline from 'readline'

function waitForInput(): Promise<void> {
  return new Promise((resolve) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    })

    console.log('Press any key to continue...')

    rl.question('', () => {
      rl.close()
      resolve()
    })
  })
}

// consts
const ARTIFACTS_PATH = '../artifacts'

function createConfig(
  chainId?: string,
  rpcEndpoint?: string,
  prefix?: string,
  gasPrice?: GasPrice,
) {
  if (
    chainId === undefined ||
    rpcEndpoint === undefined ||
    prefix === undefined
  ) {
    throw new Error('chainId, rpcEndpoint, and prefix must all be defined')
  }

  if (gasPrice === undefined) {
    gasPrice = GasPrice.fromString('0.025uosmo')
  }

  return { chainId, rpcEndpoint, prefix, gasPrice }
}

// main
async function main() {
  const mnemonic = process.env.MNEMONIC

  // just check mnemonic has actually been defined
  if (mnemonic === null || mnemonic === undefined) {
    const message = `mnemonic undefined`

    throw new Error(message)
  }

  let chainId = process.env.CHAINID
  let rpcEndpoint = process.env.RPC
  let prefix = process.env.PREFIX

  const config = createConfig(chainId, rpcEndpoint, prefix)

  const client = await setupNodeLocal(config, mnemonic)
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: config.prefix,
  })
  let deployConfig: Config = testnet
  const isTestnet = process.env.NETWORK === 'testnet'

  const [account] = await wallet.getAccounts()

  console.log(`\nWallet address from seed (owner): ${account.address}`)

  ///
  /// Upload CW20 Contract
  ///
  await waitForInput()
  console.log('Uploading CW20...')
  const cw20CodeId = await uploadContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'cw20_base.wasm'),
    'auto',
  )
  console.log(`\nCW20:\n\tCode ID: ${cw20CodeId}`)

  ///
  /// Deploy Collector Contract
  ///
  await waitForInput()
  console.log('\nDeploying Fee Collector...')
  const collectorDeployment = await deployContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'collector.wasm'),
    'collector',
    {},
    '150000',
    {},
  )

  console.log(
    `Fee Collector:
    \tContract Address: ${collectorDeployment.address}
    \tCode ID: ${collectorDeployment.codeId}`,
  )

  ///
  /// Deploy Staking Contract
  ///
  await waitForInput()
  console.log('\nDeploying Staking...')

  deployConfig.staking.fee_collector = collectorDeployment.address
  deployConfig.staking.token_code_id = cw20CodeId
  console.log(deployConfig.staking)

  const stakingDeployment = await deployContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'staking.wasm'),
    'staking',
    deployConfig.staking,
    '150000',
    {},
  )

  console.log(
    `Staking:
    \tContract Address: ${stakingDeployment.address}
    \tCode ID: ${stakingDeployment.codeId}`,
  )

  ///
  /// Deploy Distributor Contract
  ///
  await waitForInput()
  console.log('\nDeploying Distributor...')

  deployConfig.distributor.distribution.push([
    collectorDeployment.address,
    '500000',
  ])

  const distributorDeployment = await deployContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'distributor.wasm'),
    'distributor',
    deployConfig.distributor,
    '150000',
    {},
  )

  console.log(
    `Distributor:
    \tContract Address: ${distributorDeployment.address}
    \tCode ID: ${distributorDeployment.codeId}`,
  )

  ///
  /// Add token to collector
  ///
  await waitForInput()
  console.log('\nAdd token to collector...')

  await executeContract(
    client,
    account.address,
    collectorDeployment.address,
    {
      add_token: {
        token: deployConfig.staking.reward_denom,
      },
    },
    '150000',
  )

  ///
  /// Unpause staking contract
  ///
  await waitForInput()

  console.log('\nUnpause staking contract...')
  await executeContract(
    client,
    account.address,
    stakingDeployment.address,
    {
      unpause: {},
    },
    '150000',
  )

  ///
  /// Update collector contract whitelist
  ///
  await waitForInput()

  console.log('\nUpdate collector contract whitelist...')
  await executeContract(
    client,
    account.address,
    collectorDeployment.address,
    {
      update_whitelist: {
        address: stakingDeployment.address,
      },
    },
    '150000',
  )

  ///
  /// Query collector config
  ///
  await waitForInput()

  console.log('\nQuerying Collector Contract...')
  let owner = await queryContract(client, collectorDeployment.address, {
    get_owner: {},
  })
  let whitelist = await queryContract(client, collectorDeployment.address, {
    get_whitelist: {},
  })
  let tokenlist = await queryContract(client, collectorDeployment.address, {
    get_token_list: {},
  })
  console.log('Owner:\n', owner)
  console.log('Whitelist:\n', whitelist)
  console.log('Tokens:\n', tokenlist)

  ///
  /// Query distributor config
  ///
  await waitForInput()

  console.log('\nQuerying Distributor Contract...')
  owner = await queryContract(client, distributorDeployment.address, {
    get_owner: {},
  })
  let distributorConfig = await queryContract(
    client,
    distributorDeployment.address,
    {
      get_config: {},
    },
  )
  let token = await queryContract(client, distributorDeployment.address, {
    get_token: {},
  })
  console.log('Owner:\n', owner)
  console.log('Config:\n', distributorConfig)
  console.log('Token:\n', token)

  ///
  /// Query staking config
  ///
  await waitForInput()

  console.log('\nQuerying Staking Contract...')
  let state = await queryContract(client, stakingDeployment.address, {
    state: {},
  })
  let stakingConfig = await queryContract(client, stakingDeployment.address, {
    config: {},
  })
  console.log('State:\n', state)
  console.log('Config:\n', stakingConfig)

  ///
  /// Save contract address to file
  ///
  let data = {
    collectorAddress: collectorDeployment.address,
    distributorAddress: distributorDeployment.address,
    stakingAddress: stakingDeployment.address,
  }

  let jsonContent = JSON.stringify(data)

  writeFile(
    'contract-address.json',
    jsonContent,
    'utf8',
    (err: Error | null) => {
      if (err) {
        console.log('An error occurred while writing JSON Object to File.')
        return console.log(err)
      }

      console.log('JSON file has been saved.')
    },
  )

  ///
  /// Query Balances Start
  ///
  await waitForInput()

  console.log('\nQuerying all balances start...')
  let deposit_balance = await queryBalance(
    client,
    account.address,
    deployConfig.staking.deposit_denom,
  )

  let reward_balance = await queryBalance(
    client,
    account.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${account.address} (owner):\n\t${deposit_balance.amount} ${deployConfig.staking.deposit_denom}\n\t${reward_balance.amount} ${deployConfig.staking.reward_denom}`,
  )

  let deposit_balance_protocol = await queryBalance(
    client,
    deployConfig.protocol_address,
    deployConfig.staking.deposit_denom,
  )

  let reward_balance_protocol = await queryBalance(
    client,
    deployConfig.protocol_address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${deployConfig.protocol_address} (protocol contract):\n\t${deposit_balance_protocol.amount} ${deployConfig.staking.deposit_denom}\n\t${reward_balance_protocol.amount} ${deployConfig.staking.reward_denom}`,
  )

  let deposit_balance_manager = await queryBalance(
    client,
    deployConfig.manager_address,
    deployConfig.staking.deposit_denom,
  )

  let reward_balance_manager = await queryBalance(
    client,
    deployConfig.manager_address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${deployConfig.manager_address} (strategy manager):\n\t${deposit_balance_manager.amount} ${deployConfig.staking.deposit_denom}\n\t${reward_balance_manager.amount} ${deployConfig.staking.reward_denom}`,
  )

  let deposit_balance_collector = await queryBalance(
    client,
    collectorDeployment.address,
    deployConfig.staking.deposit_denom,
  )

  let reward_balance_collector = await queryBalance(
    client,
    collectorDeployment.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${collectorDeployment.address} (collector contract):\n\t${deposit_balance_collector.amount} ${deployConfig.staking.deposit_denom}\n\t${reward_balance_collector.amount} ${deployConfig.staking.reward_denom}`,
  )

  ///
  /// Deposit fees to distributor
  ///
  await waitForInput()

  console.log('\nDeposit fees into distributor...')
  let balance = await queryBalance(
    client,
    distributorDeployment.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${distributorDeployment.address} (distributor contract):\n\t${balance.amount} ${deployConfig.staking.reward_denom}`,
  )

  let tx = await sendToken(
    client,
    account.address,
    distributorDeployment.address,
    '20000000',
    deployConfig.staking.reward_denom,
  )
  console.log('\nDeposit tx:\n\t', tx.transactionHash)

  balance = await queryBalance(
    client,
    distributorDeployment.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${distributorDeployment.address}:\n\t${balance.amount} ${deployConfig.staking.reward_denom}`,
  )

  ///
  /// Distribute fees from distributor
  ///
  await waitForInput()

  console.log('\nDistribute fees from distributor...')
  let distribute_tx = await executeContract(
    client,
    account.address,
    distributorDeployment.address,
    { distribute: {} },
    'auto',
  )
  console.log('\nDistribute tx:\n\t', distribute_tx.transactionHash)

  balance = await queryBalance(
    client,
    collectorDeployment.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${distributorDeployment.address} (distributor contract):\n\t${balance.amount} ${deployConfig.staking.reward_denom}`,
  )

  balance = await queryBalance(
    client,
    collectorDeployment.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${collectorDeployment.address} (collector contract):\n\t${balance.amount} ${deployConfig.staking.reward_denom}\n\t${reward_balance_collector.amount} ${deployConfig.staking.reward_denom}`,
  )

  balance = await queryBalance(
    client,
    deployConfig.protocol_address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${account.address} (owner):\n\t${deposit_balance_protocol.amount} ${deployConfig.staking.reward_denom}\n\t${reward_balance_protocol.amount} ${deployConfig.staking.reward_denom}`,
  )

  ///
  /// Stake deposit token into staking contract
  ///
  await waitForInput()

  console.log('\nStake deposit token into staking contract...')
  balance = await queryBalance(
    client,
    stakingDeployment.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${stakingDeployment.address} (staking contract):\n\t${balance.amount} ${deployConfig.staking.reward_denom}`,
  )
  let stake_tx = await executeContract(
    client,
    account.address,
    stakingDeployment.address,
    { stake: {} },
    'auto',
    [{ denom: deployConfig.staking.deposit_denom, amount: '10000000' }],
  )
  console.log('\nStaking tx:\n\t', stake_tx.transactionHash)
  balance = await queryBalance(
    client,
    stakingDeployment.address,
    deployConfig.staking.reward_denom,
  )

  console.log(
    `\nBalances of ${stakingDeployment.address} (staking contract):\n\t${balance.amount} ${deployConfig.staking.deposit_denom}`,
  )

  console.log('\nQuerying CW20 Token User Receipt Token...')
  balance = await queryContract(client, stakingConfig.staked_denom, {
    balance: {
      address: account.address,
    },
  })
  console.log('\nCW20 Token User Receipt Token:\n\t', balance)

  console.log('\nQuerying Staking Contract User Staked Amount...')
  let position = await queryContract(client, stakingDeployment.address, {
    get_user_staked_amount: {
      user: account.address,
    },
  })
  console.log('\nStaking Contract User Staked Amount:\n\t', position)
  let claimable = await queryContract(client, stakingDeployment.address, {
    get_claimable: {
      user: account.address,
    },
  })
  console.log('\nStaking Contract User Claimable Amount:\n\t', claimable)

  ///
  /// Claim rewards
  ///
  await waitForInput()

  console.log(deployConfig.staking.reward_denom.toLowerCase())
  console.log('\nClaim rewards...')
  balance = await queryBalance(
    client,
    account.address,
    deployConfig.staking.reward_denom,
  )
  console.log(
    `\nBalances of ${account.address} (owner):\n\t${balance.amount} ${deployConfig.staking.reward_denom}`,
  )

  balance = await queryContract(client, stakingConfig.staked_denom, {
    balance: {
      address: account.address,
    },
  })
  console.log('\nCW20 Token User Receipt Token:\n\t', balance)

  let claim_tx = await executeContract(
    client,
    account.address,
    stakingDeployment.address,
    {
      claim: {
        recipient: account.address,
      },
    },
    'auto',
  )
  console.log('\nClaim rewards tx:\n', claim_tx.transactionHash)

  balance = await queryBalance(
    client,
    account.address,
    deployConfig.staking.reward_denom,
  )
  console.log(
    `\nBalances of ${account.address} (owner):\n\t${balance.amount} ${deployConfig.staking.reward_denom}`,
  )

  balance = await queryContract(client, stakingConfig.staked_denom, {
    balance: {
      address: account.address.toLowerCase(),
    },
  })
  console.log('\nCW20 Token User Receipt Token:\n\t', balance)

  ///
  /// Unstake deposit token from staking contract
  ///
  await waitForInput()

  console.log('\nUnstake deposit token from staking contract...')
  balance = await queryBalance(
    client,
    stakingDeployment.address,
    deployConfig.staking.deposit_denom,
  )

  console.log(
    `\nBalances of ${stakingDeployment.address}:\n\t${balance.amount} ${deployConfig.staking.deposit_denom}`,
  )
  let msg = {
    unstake: {},
  }

  let unstake_tx = await executeContract(
    client,
    account.address,
    stakingConfig.staked_denom,
    {
      send: {
        amount: balance.balance,
        contract: stakingDeployment.address,
        msg: toUtf8(JSON.stringify(msg)),
      },
    },
    'auto',
  )
  console.log('\nUnstaking tx:\n\t', unstake_tx.transactionHash)
  balance = await queryBalance(
    client,
    stakingDeployment.address,
    deployConfig.staking.deposit_denom,
  )

  console.log(
    `\nBalances of ${stakingDeployment.address} (staking contract):\n\t${balance.amount} ${deployConfig.staking.deposit_denom}`,
  )

  console.log('\nQuerying CW20 Token User Receipt Token...')
  balance = await queryContract(client, stakingConfig.staked_denom, {
    balance: {
      address: account.address,
    },
  })
  console.log('\nCW20 Token User Receipt Token:\n\t', balance)
}

main().catch(console.log)
