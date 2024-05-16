/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { InstantiateMsg, ExecuteMsg, Uint128, TokenUnverified, RawPriceItem, FundsForTokenUnverified, CurrencyForTokenUnverified, QueryMsg, MigrateMsg, Uint64, Addr, Config, Token, ArrayOfPriceItem, PriceItem, FundsForToken, CurrencyForToken } from "./Oracle.types";
export interface OracleMsg {
  contractAddress: string;
  sender: string;
  acceptAdminRole: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateConfig: ({
    admin,
    scheduler,
    worker
  }: {
    admin?: string;
    scheduler?: string;
    worker?: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updatePrices: ({
    data
  }: {
    data: RawPriceItem[];
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class OracleMsgComposer implements OracleMsg {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.acceptAdminRole = this.acceptAdminRole.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.updatePrices = this.updatePrices.bind(this);
  }

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
    scheduler,
    worker
  }: {
    admin?: string;
    scheduler?: string;
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
            scheduler,
            worker
          }
        })),
        funds: _funds
      })
    };
  };
  updatePrices = ({
    data
  }: {
    data: RawPriceItem[];
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_prices: {
            data
          }
        })),
        funds: _funds
      })
    };
  };
}