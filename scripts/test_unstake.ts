import "dotenv/config.js";
import { queryContract } from "./helpers.js";
import { GasPrice, setupNodeLocal } from "cosmwasm";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { toUtf8, toBase64 } from "@cosmjs/encoding";
import readline from "readline";

function waitForInput(question: string = ""): Promise<string> {
  return new Promise((resolve) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    if (question === "") {
      console.log("Press any key to continue...");
    }
    rl.question(question, (answer: string) => {
      rl.close();
      resolve(answer);
    });
  });
}

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

const jsonToBinary = (json: Record<string, unknown>): string => {
  return toBase64(toUtf8(JSON.stringify(json)));
};

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
  const isTestnet = process.env.NETWORK === "testnet";

  const [account] = await wallet.getAccounts();

  console.log(`\nWallet address from seed (owner): ${account.address}`);

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
  let amount_to_unstake: string;
  // keep asking for input until we get a valid amount
  while (
    isNaN(
      Number(
        (amount_to_unstake = await waitForInput(
          "\nHow many tokens would you like to unstake (in untrn)?\n"
        ))
      )
    )
  ) {
    console.log("Invalid amount, try again.");
  }

  let msg = {
    unstake: {},
  };

  let unstake_tx = await client.execute(
    account.address,
    stakingConfig.staked_denom,
    {
      send: {
        contract: stakingDeploymentAddress,
        amount: amount_to_unstake,
        msg: jsonToBinary(msg),
      },
    },
    "auto" // Will simulate and estimate gas automatically
    // { amount: [{ denom: "untrn", amount: "8200" }], gas: "410000" } // Force broadcasting to the blockchain
  );

  console.log("\nUnstaking tx:\n\t", unstake_tx.transactionHash);
}

main().catch(console.log);
