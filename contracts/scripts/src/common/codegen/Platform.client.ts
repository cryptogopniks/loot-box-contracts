/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.9.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Uint128, Decimal, InstantiateMsg, WeightInfo, ExecuteMsg, NftInfoForString, QueryMsg, MigrateMsg, Addr, Balance, NftInfoForAddr, BoxStats, OpeningInfo, Config, UserInfo, ArrayOfQueryUserListResponseItem, QueryUserListResponseItem } from "./Platform.types";
export interface PlatformReadOnlyInterface {
  contractAddress: string;
  queryConfig: () => Promise<Config>;
  queryBoxStats: () => Promise<BoxStats>;
  queryBalance: () => Promise<Balance>;
  queryUser: ({
    address
  }: {
    address: string;
  }) => Promise<UserInfo>;
  queryUserList: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<ArrayOfQueryUserListResponseItem>;
}
export class PlatformQueryClient implements PlatformReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;
  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.queryConfig = this.queryConfig.bind(this);
    this.queryBoxStats = this.queryBoxStats.bind(this);
    this.queryBalance = this.queryBalance.bind(this);
    this.queryUser = this.queryUser.bind(this);
    this.queryUserList = this.queryUserList.bind(this);
  }
  queryConfig = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_config: {}
    });
  };
  queryBoxStats = async (): Promise<BoxStats> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_box_stats: {}
    });
  };
  queryBalance = async (): Promise<Balance> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_balance: {}
    });
  };
  queryUser = async ({
    address
  }: {
    address: string;
  }): Promise<UserInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_user: {
        address
      }
    });
  };
  queryUserList = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<ArrayOfQueryUserListResponseItem> => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_user_list: {
        limit,
        start_after: startAfter
      }
    });
  };
}
export interface PlatformInterface extends PlatformReadOnlyInterface {
  contractAddress: string;
  sender: string;
  buy: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  open: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  claim: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  send: ({
    amount,
    recipient
  }: {
    amount: Uint128;
    recipient: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  acceptAdminRole: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  deposit: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  depositNft: ({
    nftInfoList
  }: {
    nftInfoList: NftInfoForString[];
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateConfig: ({
    admin,
    boxPrice,
    denom,
    distribution,
    worker
  }: {
    admin?: string;
    boxPrice?: Uint128;
    denom?: string;
    distribution?: WeightInfo[];
    worker?: string;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  lock: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  unlock: (fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  withdraw: ({
    amount
  }: {
    amount: Uint128;
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawNft: ({
    nftInfoList
  }: {
    nftInfoList: NftInfoForString[];
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
  updateNftPrice: ({
    nftInfoList
  }: {
    nftInfoList: NftInfoForString[];
  }, fee?: number | StdFee | "auto", memo?: string, _funds?: Coin[]) => Promise<ExecuteResult>;
}
export class PlatformClient extends PlatformQueryClient implements PlatformInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;
  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.buy = this.buy.bind(this);
    this.open = this.open.bind(this);
    this.claim = this.claim.bind(this);
    this.send = this.send.bind(this);
    this.acceptAdminRole = this.acceptAdminRole.bind(this);
    this.deposit = this.deposit.bind(this);
    this.depositNft = this.depositNft.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.lock = this.lock.bind(this);
    this.unlock = this.unlock.bind(this);
    this.withdraw = this.withdraw.bind(this);
    this.withdrawNft = this.withdrawNft.bind(this);
    this.updateNftPrice = this.updateNftPrice.bind(this);
  }
  buy = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      buy: {}
    }, fee, memo, _funds);
  };
  open = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      open: {}
    }, fee, memo, _funds);
  };
  claim = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      claim: {}
    }, fee, memo, _funds);
  };
  send = async ({
    amount,
    recipient
  }: {
    amount: Uint128;
    recipient: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      send: {
        amount,
        recipient
      }
    }, fee, memo, _funds);
  };
  acceptAdminRole = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      accept_admin_role: {}
    }, fee, memo, _funds);
  };
  deposit = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      deposit: {}
    }, fee, memo, _funds);
  };
  depositNft = async ({
    nftInfoList
  }: {
    nftInfoList: NftInfoForString[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      deposit_nft: {
        nft_info_list: nftInfoList
      }
    }, fee, memo, _funds);
  };
  updateConfig = async ({
    admin,
    boxPrice,
    denom,
    distribution,
    worker
  }: {
    admin?: string;
    boxPrice?: Uint128;
    denom?: string;
    distribution?: WeightInfo[];
    worker?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_config: {
        admin,
        box_price: boxPrice,
        denom,
        distribution,
        worker
      }
    }, fee, memo, _funds);
  };
  lock = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      lock: {}
    }, fee, memo, _funds);
  };
  unlock = async (fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      unlock: {}
    }, fee, memo, _funds);
  };
  withdraw = async ({
    amount
  }: {
    amount: Uint128;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw: {
        amount
      }
    }, fee, memo, _funds);
  };
  withdrawNft = async ({
    nftInfoList
  }: {
    nftInfoList: NftInfoForString[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_nft: {
        nft_info_list: nftInfoList
      }
    }, fee, memo, _funds);
  };
  updateNftPrice = async ({
    nftInfoList
  }: {
    nftInfoList: NftInfoForString[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, _funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_nft_price: {
        nft_info_list: nftInfoList
      }
    }, fee, memo, _funds);
  };
}