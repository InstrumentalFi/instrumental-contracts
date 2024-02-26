import 'dotenv/config.js';
import {
  deployContract,
  executeContract,
  queryContract,
  uploadContract,
} from './helpers.js';
import { DirectSecp256k1HdWallet, OfflineSigner } from '@cosmjs/proto-signing';
import { writeFile } from 'fs';
import { testnet } from './deploy_configs.js';
import { join } from 'path';

import { GasPrice } from '@cosmjs/stargate';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';

// consts
const ARTIFACTS_PATH = '../artifacts';

async function createWallet(
  mnemonic: string,
  prefixValue: string,
): Promise<OfflineSigner> {
  // const mnemonic =
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: prefixValue,
  });

  return wallet;
}
// main
async function main() {
  const mnemonic = process.env.MNEMONIC;

  // just check mnemonic has actually been defined
  if (mnemonic === null || mnemonic === undefined) {
    const message = `mnemonic undefined`;

    throw new Error(message);
  }

  let chainId = process.env.CHAINID;
  let rpcEndpoint = process.env.RPC;
  let prefix = process.env.PREFIX;
  if (
    chainId === undefined ||
    rpcEndpoint === undefined ||
    prefix === undefined
  ) {
    throw new Error('chainId, rpcEndpoint, and prefix must all be defined');
  }

  let deployConfig: Config = testnet;
  const isTestnet = process.env.NETWORK === 'testnet';

  if (!rpcEndpoint) return;

  const wallet = await createWallet(mnemonic, prefix);

  const [account] = await wallet.getAccounts();
  console.log(`Wallet address from seed: ${account.address}`);

  const client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    wallet,
    { gasPrice: GasPrice.fromString('0.025uosmo') },
  );

  ///
  /// Upload CW20 Contract
  ///
  console.log('Uploading CW20...');
  const cw20CodeId = await uploadContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'cw20_base.wasm'),
    'auto',
  );
  console.log(`CW20:\n\tCode ID: ${cw20CodeId}`);

  ///
  /// Deploy Collector Contract
  ///
  console.log('Deploying Fee Collector...');
  const collectorDeployment = await deployContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'collector.wasm'),
    'collector',
    {},
    '150000',
    {},
  );

  console.log(
    `Fee Collector:
    \tContract Address: ${collectorDeployment.address}
    \tCode ID: ${collectorDeployment.codeId}`,
  );

  ///
  /// Deploy Staking Contract
  ///
  console.log('Deploying Staking...');

  deployConfig.staking.fee_collector = collectorDeployment.address;
  deployConfig.staking.token_code_id = cw20CodeId;
  console.log(deployConfig.staking);

  const stakingDeployment = await deployContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'staking.wasm'),
    'staking',
    deployConfig.staking,
    '150000',
    {},
  );

  console.log(
    `Staking:
    \tContract Address: ${stakingDeployment.address}
    \tCode ID: ${stakingDeployment.codeId}`,
  );

  ///
  /// Deploy Distributor Contract
  ///
  console.log('Deploying Distributor...');

  deployConfig.distributor.distribution.push([
    collectorDeployment.address,
    '500000',
  ]);

  const distributorDeployment = await deployContract(
    client,
    account.address,
    join(ARTIFACTS_PATH, 'distributor.wasm'),
    'distributor',
    deployConfig.distributor,
    '150000',
    {},
  );

  console.log(
    `Distributor:
    \tContract Address: ${distributorDeployment.address}
    \tCode ID: ${distributorDeployment.codeId}`,
  );

  ///
  /// Add token to collector
  ///
  console.log('Add token to collector...');
  await executeContract(
    client,
    account.address,
    collectorDeployment.address,
    {
      add_token: {
        token: 'uosmo',
      },
    },
    '150000',
  );

  ///
  /// Unpause staking contract
  ///
  console.log('Unpause staking contract...');
  await executeContract(
    client,
    account.address,
    stakingDeployment.address,
    {
      unpause: {},
    },
    '150000',
  );

  ///
  /// Query staking config
  ///
  console.log('Querying Staking Contract...');
  let state = await queryContract(client, stakingDeployment.address, {
    state: {},
  });
  let stakingConfig = await queryContract(client, stakingDeployment.address, {
    config: {},
  });
  console.log('State:\n', state);
  console.log('Config:\n', stakingConfig);

  ///
  /// Query collector config
  ///
  console.log('Querying Collector Contract...');
  let owner = await queryContract(client, collectorDeployment.address, {
    get_owner: {},
  });
  let whitelist = await queryContract(client, collectorDeployment.address, {
    get_whitelist: {},
  });
  let tokenlist = await queryContract(client, collectorDeployment.address, {
    get_token_list: {},
  });
  console.log('Owner:\n', owner);
  console.log('Whitelist:\n', whitelist);
  console.log('Tokens:\n', tokenlist);

  ///
  /// Query distributor config
  ///
  console.log('Querying Distributor Contract...');
  owner = await queryContract(client, distributorDeployment.address, {
    get_owner: {},
  });
  let distributorConfig = await queryContract(
    client,
    distributorDeployment.address,
    {
      get_config: {},
    },
  );
  let token = await queryContract(client, distributorDeployment.address, {
    get_token: {},
  });
  console.log('Owner:\n', owner);
  console.log('Config:\n', distributorConfig);
  console.log('Token:\n', token);

  ///
  /// Save contract address to file
  ///
  let data = {
    collectorAddress: collectorDeployment.address,
    distributorAddress: distributorDeployment.address,
    stakingAddress: stakingDeployment.address,
  };

  let jsonContent = JSON.stringify(data);

  writeFile(
    'contract-address.json',
    jsonContent,
    'utf8',
    (err: Error | null) => {
      if (err) {
        console.log('An error occurred while writing JSON Object to File.');
        return console.log(err);
      }

      console.log('JSON file has been saved.');
    },
  );
}

main().catch(console.log);
