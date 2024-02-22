// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Deeplink } from './Deeplink'
import type { Images } from './Images'
import type { Network } from './Network'
import type { Platform } from './Platform'
import type { Version } from './Version'
import type { WalletType } from './WalletType'

export interface WalletMetadata {
  slug: string
  name: string
  description?: string
  homepage?: string
  chains?: Array<Network>
  version?: Version
  walletType: WalletType
  mobile: Deeplink | null
  desktop: Deeplink | null
  image: Images
  app?: Record<Platform, string>
  injectPath?: Record<Network, string>
  lastUpdatedTimestamp?: bigint
}
