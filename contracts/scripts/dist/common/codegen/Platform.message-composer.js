/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.9.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
export class PlatformMsgComposer {
  constructor(sender, contractAddress) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.noisReceive = this.noisReceive.bind(this);
    this.acceptAdminRole = this.acceptAdminRole.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.requestBoxList = this.requestBoxList.bind(this);
  }
  noisReceive = ({
    callback
  }, _funds) => {
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
  acceptAdminRole = _funds => {
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
  }, _funds) => {
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
  requestBoxList = _funds => {
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