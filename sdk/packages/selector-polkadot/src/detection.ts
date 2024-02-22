import { type Injected, type InjectedExtension } from '@polkadot/extension-inject/types'
import { type WalletIcon } from '@wallet-standard/core'
import { appToIcon } from './tempIcons'
import { WalletMetadata } from '@nightlylabs/nightly-connect-polkadot'
export interface PolkadotWalletInjected {
  // Default Polkadot standard
  connect?: (origin: string) => Promise<InjectedExtension> // Is this even used ?
  enable?: (origin: string) => Promise<Injected>
  version?: string
  // Custom should be provided by the wallet
  name: string
  slug: string
  icon?: WalletIcon
}

declare global {
  interface Window {
    injectedWeb3?: { [key in string]: PolkadotWalletInjected }
  }
}
export const getPolkadotWallets = (): PolkadotWalletInjected[] => {
  if (window && window.injectedWeb3) {
    return Object.entries(window.injectedWeb3).map(([key, value]) => ({
      ...value,
      name: value.name ?? key, // value.name might be undefined
      slug: key,
      icon: value.icon ?? appToIcon[key] ?? 'https://registry.nightly.app/networks/polkadot.png' // TODO add default icon
    }))
  } else {
    return []
  }
}

export interface IPolkadotWalletListItem extends WalletMetadata {
  recent?: boolean
  detected?: boolean
  injectedWallet?: PolkadotWalletInjected
}

export const getPolkadotWalletsList = (presetList: WalletMetadata[], recentWalletName?: string) => {
  const windowWallets = getPolkadotWallets()

  const walletsData: Record<string, IPolkadotWalletListItem> = {}

  presetList.forEach((wallet) => {
    walletsData[wallet.slug.toLocaleLowerCase()] = {
      ...wallet,
      recent: recentWalletName === wallet.name
    }
  })
  for (const wallet of windowWallets) {
    // Check if wallet is already in the list
    // by namespace
    if (walletsData[wallet.slug.toLocaleLowerCase()]) {
      walletsData[wallet.slug.toLocaleLowerCase()] = {
        ...(walletsData?.[wallet.slug.toLocaleLowerCase()] ?? {
          name: wallet.name,
          icon: wallet.icon,
          link: '',
          deeplink: null,
          recent: recentWalletName === wallet.name,
          walletType: 'hybrid'
        }),
        detected: true,
        injectedWallet: wallet
      }
      continue
    }

    // Check if wallet is already in the list
    // by name
    if (walletsData[wallet.name.toLocaleLowerCase()]) {
      walletsData[wallet.name.toLocaleLowerCase()] = {
        ...(walletsData?.[wallet.name.toLocaleLowerCase()] ?? {
          name: wallet.name,
          icon: wallet.icon,
          link: '',
          deeplink: null,
          recent: recentWalletName === wallet.name,
          walletType: 'hybrid'
        }),
        detected: true,
        injectedWallet: wallet
      }
      continue
    }
    walletsData[wallet.name.toLocaleLowerCase()] = {
      slug: wallet.name,
      name: wallet.name,
      image: {
        default: wallet.icon as string,
        sm: wallet.icon as string,
        md: wallet.icon as string,
        lg: wallet.icon as string
      },
      desktop: null,
      mobile: null,
      recent: recentWalletName === wallet.name,
      detected: true,
      injectedWallet: wallet,
      walletType: 'hybrid'
    }
  }

  return Object.values(walletsData)
}
