import { Wallet } from '@wallet-standard/core'

import { isWalletWithRequiredFeatureSet } from '@mysten/wallet-standard'

export const suiWalletsFilter = (wallet: Wallet) => {
  const is = isWalletWithRequiredFeatureSet(wallet, [
    'sui:signAndExecuteTransactionBlock',
    'sui:signTransactionBlock'
  ])
  console.log(wallet)
  console.log(is)
  return is
}
