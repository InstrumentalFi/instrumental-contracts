import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { Coin } from 'cosmwasm'
import { readFileSync } from 'fs'

interface Opts {
  admin?: string
  coins?: string
}

export async function uploadContract(
  client: SigningCosmWasmClient,
  senderAddress: string,
  filepath: string,
  feeAmount: string,
) {
  const contract = readFileSync(filepath)

  let code_id = await client.upload(senderAddress, contract, 'auto')

  return Number(code_id.codeId) // code_id
}

export async function instantiateContract(
  client: SigningCosmWasmClient,
  senderAddress: string,
  codeId: number,
  label: string,
  msg: Record<string, unknown>,
  feeAmount: string,
  opts: Opts = {},
) {
  let admin = opts.admin
  if (admin == undefined) {
    admin = senderAddress
  }

  let result = await client.instantiate(
    senderAddress,
    codeId,
    msg,
    label,
    'auto',
    opts,
  )
  return result.contractAddress // contract address
}

export async function executeContract(
  client: SigningCosmWasmClient,
  senderAddress: string,
  contractAddress: string,
  msg: Record<string, unknown>,
  feeAmount: string,
  funds?: Coin[],
) {
  const result = await client.execute(
    senderAddress,
    contractAddress,
    msg,
    'auto',
    undefined,
    funds,
  )

  return result
}

export async function queryContract(
  client: SigningCosmWasmClient,
  contractAddress: string,
  query: Record<string, unknown>,
): Promise<any> {
  let result = await client.queryContractSmart(contractAddress, query)
  console.log(result)
  return result
}

export async function deployContract(
  client: SigningCosmWasmClient,
  senderAddress: string,
  filepath: string,
  label: string,
  initMsg: Record<string, unknown>,
  feeAmount: string,
  opts: object,
) {
  const codeId = await uploadContract(
    client,
    senderAddress,
    filepath,
    feeAmount,
  )

  const address = await instantiateContract(
    client,
    senderAddress,
    codeId,
    label,
    initMsg,
    feeAmount,
    opts,
  )

  console.log(codeId)

  return { address, codeId }
}

export async function sendToken(
  client: SigningCosmWasmClient,
  senderAddress: string,
  recipientAddress: string,
  amount: string,
  denom: string,
) {
  return await client.sendTokens(
    senderAddress,
    recipientAddress,
    [{ denom: denom, amount: amount }],
    'auto',
  )
}
