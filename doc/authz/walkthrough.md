# Autocompounding via Authz walkthrough

This is a walkthrough of a non-custodial Authz based approach to autocompounding
Osmosis LP Rewards. It is a working proof-of-concept and outlines some of the
limitations in relation to claiming a protocol fee.

In this scenario we have the following actors:

1. Alice - A user wanting to autocompound Internal and External rewards
2. Bot - A bot that autocompounds Alice's position

## Osmosis LP Types

Osmosis has the following types of Pools

- Stableswap
- Weighted
- Supercharged

Supercharged pools are a new feature of Osmosis, allowing users to provide
liquidity within specific ranges, also known as concentrated liquidity.
Autocompounding these positions is complex and sensitive for users providing
liquidity within a range that could move so for the purpose of this PoC this
type of pool is out of scope.

Osmosis has the following types of Incentives

- Internal incentives - This is enabled by Governance for specific pools and
  emits OSMO shortly after the end of the epoch. OSMO is send directly to the
  user and no user action is required.
- External incentives - These rewards may be added by anyone to guages relating
  to specific LP pools. These rewards are also send directly to the user shortly
  after the epoch.
- Superfluid - This allows a portion of OSMO tokens that underlie your LP
  position to be bonded to a validator. Superfluid rewards are also send
  directly to the user address.

## Authz Grants

In order to autocompound, Alice must grant the following permissions to the bot
account.

- MsgJoinSwapExternAmountIn
- MsgJoinPool
- MsgLockTokens
- MsgLockAndSuperfluidDelegate

In terms of user experience, this can be handled by the front-end application.
Keplr supports showing this to the user when signing the transaction.

![Keplr grants][12]

Enabling grants can also be executed using `osmosisd`. Here
`osmo2yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr` is the bot account.

```sh
osmosisd tx authz grant osmo2yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr generic \
    --msg-type=/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn \
    --from=alice
osmosisd tx authz grant osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr generic \
    --msg-type=/osmosis.gamm.v1beta1.MsgJoinPool \
    --from=alice
osmosisd tx authz grant osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr generic \
    --msg-type=/osmosis.lockup.MsgLockTokens \
    --from=alice
osmosisd tx authz grant osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr generic \
    --msg-type=/osmosis.superfluid.MsgLockAndSuperfluidDelegate \
    --from=alice
```

Here are the osmosis-testnet transactions for these grants:

- [MsgJoinSwapExternAmountIn][1]
- [MsgJoinPool][2]
- [MsgLockTokens][3]
- [MsgLockAndSuperfluidDelegatea[4]

Grants by default are 1 year, but can be any period of time and can be revoked
from either from `osmosisd` or Keplr at any time.

```sh
osmosisd tx authz revoke \
    osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr \
    /osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn --from=alice
```

Grants can be seen and revoked within Keplr under Settings > General > Manage
Authz

## Autocompounding

In this PoC we will assume that 10uosmo has been rewarded to Alice at the end of
the epoch for providing liquidity to OSMO/ION pool and that she wishes to
autocompound this back into the OSMO/ION pool.

The bot will use the Authz grant to perform this on her behalf.

First an unsigned transation is generated to at the 10uosmo to the pool.

```sh
osmosisd tx gamm join-swap-extern-amount-in \
    10uosmo --pool-id 1 100000 \
    --from osmo1tj0zhw2sqtvy43gzjqh6r6wff9zykfwcyut3ez \
    --generate-only > tx.json
```

This is signed by the bot and broadcast

```sh
osmosisd tx authz exec tx.json --from bot
```

This successfully executes an [Authz transaction][5] to deposit the 10uosmo into
the LP pool position. Note that the transaction can be seen under the bot
account but not Alice's account. We can see from the transaction receipt that
Alice received 49153241248 gamm/pool/1 tokens from this transaction.

We can build and sign another Authz transaction from the bot to lock these for
rewards on behalf of Alice.

```sh
osmosisd tx lockup lock-tokens \
    49153241248gamm/pool/1 --duration="24h" \
    --from osmo1tj0zhw2sqtvy43gzjqh6r6wff9zykfwcyut3ez \
    --generate-only > lock.json
```

This is signed by the bot and broadcast

```sh
osmosisd tx authz exec lock.json --from bot
```

This successfully executes an [Authz transaction][6] on behalf of Alice and
completes the autocompounding.

## Taking a protocol fee

Taking a protocol fee in this process can be achieved by adding a
SendAuthorization Authz grant for the bot for the reward token. Unfortunately
this must be done for each reward token in the pool and there may be multiple.
For the ION/OSMO pool the reward is in OSMO so we can create an Authz
SendAuthorization grant for the bot account. So it is likely the front end would
need to understand which pools will be supported and request Authz grants when
users begin autocompounding.

```sh
osmosisd tx authz grant \
    osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr send \
    --spend-limit 5uosmo --from alice
```

This results in a [successful transaction][8] and the bot is now authorized to
move a maximum of 5 uosmo from Alice's account.

```sh
osmosisd query authz grantee-grants osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr
grants:
- authorization:
    '@type': /cosmos.bank.v1beta1.SendAuthorization
    spend_limit:
    - amount: "5"
      denom: uosmo
  expiration: "2024-08-03T09:46:52Z"
  grantee: osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr
  granter: osmo1tj0zhw2sqtvy43gzjqh6r6wff9zykfwcyut3ez
```

The bot can now move funds from Alice's account

```sh
osmosisd tx bank send \
    osmo1tj0zhw2sqtvy43gzjqh6r6wff9zykfwcyut3ez \
    osmo1yrg6daqkxyeqye4aac09stzvvwppqwls6wgpxr \
    2uosmo --generate-only > transfer.json

osmosisd tx authz exec transfer.json --from bot
```

The [resulting transaction][9] shows funds being moved from Alice's account.

If the bot tries to move more than the allowance it has the [transaction will
fail][10].

Note that the send authorization is somewhat of a security risk in that whoever
controls the bot account can now transfer funds from Alice's account to the
limit specified.

```sh
failed to execute message; message index: 0: requested amount is more than spend limit: insufficient funds
```

This technique can be used to send the fee to any address (a contract) which can
then liquidate and send the fee via IBC to Centauri. It may also be possible to
do the IBC transfer directly from Authz grant but seeing as rewards tokens could
be difficult this could be complex.

## Challenges

There is no native way to get historical LP rewards sent to users. We have asked
in both the Osmosis developer group and a private channel that we have with
Osmosis. Our conclusion is that we either would need to create an indexer or use
an external data provider.

The following data source is community based project

https://api-osmosis-chain.imperator.co/lp/v1/rewards/token/{walletAddress}
https://api-osmosis-chain.imperator.co/lp/v1/rewards/historical/{walletAddress}/{token}

If we look at the an address that is using Yieldmos we can get tokens that have
been rewarded to this address

```sh
curl https://api-osmosis-chain.imperator.co/lp/v1/rewards/token/osmo1rj4k76hr8u3ra60crjpggt483x0tl60ukexs30 | jq .
```

```json
[
  {
    "token": "ASVT"
  },
  {
    "token": "LUNC"
  },
  {
    "token": "BOOT"
  },
  {
    "token": "SOMM"
  },
  {
    "token": "ECH"
  },
  {
    "token": "CRBRUS"
  },
  {
    "token": "SCRT"
  },
  {
    "token": "JKL"
  },
  {
    "token": "OSMO"
  },
  {
    "token": "GLTO"
  },
  {
    "token": "GKEY"
  },
  {
    "token": "EVMOS"
  }
]
```

We can then use the second call to get rewarded OSMO amounts

```sh
curl https://api-osmosis-chain.imperator.co/lp/v1/rewards/historical/osmo1rj4k76hr8u3ra60crjpggt483x0tl60ukexs30/OSMO | jq .
```

```json
[
    {
        "amount":5.389236,"day":"2023-08-02"}
    },
    ...etc
]
```

This amount looks to be in dollars rather than the token amount and there is no
split on the different pools that a user might receive OSMO rewards from. So
unfortunately this data source does not provide what we need.

Yieldmos partner with Dexmos who have a porition of the data provided by
Imperator.co so it may be an option to reach out to Imperator to see if they
have an api with rewards emitted to an account by date, denom and pool.

The other option is to build an indexer to harvest this data which seems a lot o
work.

## Conclusions

- It is technically possible to create an autocompounding product that takes a
  fee using Authz
- To take the fee a SendAuthorization grant must be given for each reward token
  so the front-end probably needs to know about which pools are supported.
- The bot account will have control over funds authorized via SendAuthorization
  so this could represent a security risk.
- Superfluid LP positions are hard to autocompound since the range affects
  profitability for users.
- There is an [open issue][11] around adding fee capture to Authz which has been
  accepted but not implemented.
- There is no first-party API to get historical rewards sent to users. This must
  be possible as Yieldmos must have a way to understand this. It is suggested
  that we open a dialogue with Imperator if this is seen as a viable product.
- Fees can be sent to another contract and this contract can liquidate and send
  via IBC to Centauri. Another option is to send protocol fees directly via IBC
  but since rewards can be in multiple tokens this is thought to be sub-optimal.
- This product is more complex than first thought.

## References

- [Yieldmos account][7] - you can see autocompounding transactions (and
  failures!) here.
- [Authz Fee Capture Request][11] - This has been accepted as a proposal but not
  implemented.

[1]: https://testnet.mintscan.io/osmosis-testnet/txs/B7143AE38C7C8FB497271A78A8E0FC9A930A3FE46704F7CC928595E5D01C7EFE?height=1988844
[2]: https://testnet.mintscan.io/osmosis-testnet/txs/0F8BF3DC09E62598DF1182818CB49A1989F1194485F6BB951018869AA4142A70?height=1988845
[3]: https://testnet.mintscan.io/osmosis-testnet/txs/C502B96BA0FD400EA373BEBD9DC025E1270C49A9A8227E7BC899208CB8AD3168?height=1988968
[4]: https://testnet.mintscan.io/osmosis-testnet/txs/9F8E20338FAFCBA8F562327E21E94472DE12CDE22C436165AC2D668C81F69149?height=1988982
[5]: https://testnet.mintscan.io/osmosis-testnet/txs/1169B95116CA32CFA9704F39623C468EA8DF1B06DFE805F16634F3CFC4F7027C?height=1989500
[6]: https://testnet.mintscan.io/osmosis-testnet/txs/E83C30F32978688E109893B7C6D597D61090AE591F78E38EAAEB66517C335E72?height=1989645
[7]: https://www.mintscan.io/osmosis/transactions/3C7A8CF9B4A170E354159204D8543A545BD7FD47E418097CE0C52492A5A9E04F
[8]: https://testnet.mintscan.io/osmosis-testnet/txs/F9CAC58F146FEAF9C8FA83D35525F13725A01D484A1CA55DC33E84096C12FF22?height=1989850
[9]: https://testnet.mintscan.io/osmosis-testnet/txs/151B8A78EF04BCF154A29CE7B294F094616A9795A82EF2627019E112B1FDC026?height=1989942
[10]: https://testnet.mintscan.io/osmosis-testnet/txs/D497970DDCB19596311AB43E406BC4FB285E31803DE78A26417777F2888585F9?height=1989987
[11]: https://github.com/cosmos/cosmos-sdk/issues/11583
[12]: img/grants.png
