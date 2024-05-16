/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.9.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { Uint128, Decimal, InstantiateMsg, ExecuteMsg, Timestamp, Uint64, HexBinary, NoisCallback, QueryMsg, MigrateMsg, BoxList, Addr, Config } from "./Platform.types";
export interface PlatformMsg {
  contractAddress: string;
  sender: string;
  noisReceive: ({
    callback
  }: {
    callback: NoisCallback;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  acceptAdminRole: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateConfig: ({
    admin,
    boxListLength,
    boxPrice,
    priceAndWeightList,
    proxy,
    worker
  }: {
    admin?: string;
    boxListLength?: number;
    boxPrice?: Uint128;
    priceAndWeightList?: Uint128[][][];
    proxy?: string;
    worker?: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  requestBoxList: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class PlatformMsgComposer implements PlatformMsg {
  sender: string;
  contractAddress: string;
  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.noisReceive = this.noisReceive.bind(this);
    this.acceptAdminRole = this.acceptAdminRole.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.requestBoxList = this.requestBoxList.bind(this);
  }
  noisReceive = ({
    callback
  }: {
    callback: NoisCallback;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          nois_receive: {
            callback
          }
        })),
        funds: _funds
      })
    };
  };
  acceptAdminRole = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          accept_admin_role: {}
        })),
        funds: _funds
      })
    };
  };
  updateConfig = ({
    admin,
    boxListLength,
    boxPrice,
    priceAndWeightList,
    proxy,
    worker
  }: {
    admin?: string;
    boxListLength?: number;
    boxPrice?: Uint128;
    priceAndWeightList?: Uint128[][][];
    proxy?: string;
    worker?: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_config: {
            admin,
            box_list_length: boxListLength,
            box_price: boxPrice,
            price_and_weight_list: priceAndWeightList,
            proxy,
            worker
          }
        })),
        funds: _funds
      })
    };
  };
  requestBoxList = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          request_box_list: {}
        })),
        funds: _funds
      })
    };
  };
}