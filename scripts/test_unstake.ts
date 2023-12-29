import "dotenv/config.js";
import { executeContract, queryContract } from "./helpers.js";
import { GasPrice, setupNodeLocal, Coin } from "cosmwasm";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { toUtf8, toBase64 } from "@cosmjs/encoding";
import { testnet } from "./deploy_configs.js";
import readline from "readline";

function waitForInput(): Promise<void> {
  return new Promise((resolve) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    console.log("Press any key to continue...");

    rl.question("", () => {
      rl.close();
      resolve();
    });
  });
}

// consts
const ARTIFACTS_PATH = "../artifacts";

function createConfig(
  chainId?: string,
  rpcEndpoint?: string,
  prefix?: string,
  gasPrice?: GasPrice
) {
  if (
    chainId === undefined ||
    rpcEndpoint === undefined ||
    prefix === undefined
  ) {
    throw new Error("chainId, rpcEndpoint, and prefix must all be defined");
  }

  if (gasPrice === undefined) {
    gasPrice = GasPrice.fromString("0.025untrn");
  }

  return { chainId, rpcEndpoint, prefix, gasPrice };
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

  const config = createConfig(chainId, rpcEndpoint, prefix);

  const client = await setupNodeLocal(config, mnemonic);
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: config.prefix,
  });
  let deployConfig: Config = testnet;
  const isTestnet = process.env.NETWORK === "testnet";

  const [account] = await wallet.getAccounts();

  console.log(`\nWallet address from seed (owner): ${account.address}`);

  // ///
  // /// Query staking config
  // ///
  // await waitForInput();

  const stakingDeploymentAddress =
    "neutron1qjxme67x6nq3j623kjs5utjwd20npxhs2493fpgmxa05868y4pmqqhe06w";

  console.log("\nQuerying Staking Contract...");
  let state = await queryContract(client, stakingDeploymentAddress, {
    state: {},
  });
  let stakingConfig = await queryContract(client, stakingDeploymentAddress, {
    config: {},
  });
  console.log("State:\n", state);
  console.log("Config:\n", stakingConfig);
  await waitForInput();

  let msg = {
    unstake: {},
  };
  const jsonToBinary = (json: Record<string, unknown>): string => {
    return toBase64(toUtf8(JSON.stringify(json)));
  };
  let unstake_tx = await client.execute(
    account.address,
    stakingConfig.staked_denom,
    {
      send: {
        contract:
          "neutron1qjxme67x6nq3j623kjs5utjwd20npxhs2493fpgmxa05868y4pmqqhe06w",
        amount: "1000",
        msg: jsonToBinary(msg),
      },
    },
    "auto" // Will simulate and estimate gas automatically
    //{ amount: [{ denom: "untrn", amount: "35050" }], gas: "232111" } // Force broadcasting to the blockchain
  );

  // const result = await client.execute(
  //   senderAddress,
  //   contractAddress,
  //   { send: { recipient, amount, msg: jsonToBinary(msg) } },
  //   fee,
  // )

  //   &Cw20ExecuteMsg::Send {
  //     contract: staking_address,
  //     amount: amount_to_unstake.into(),
  //     msg: to_binary(&Cw20HookMsg::Unstake {}).unwrap(),
  // },
  console.log("\nUnstaking tx:\n\t", unstake_tx.transactionHash);
}

main().catch(console.log);
