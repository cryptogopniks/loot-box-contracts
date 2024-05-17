/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { RawPriceItem } from "./Oracle.types";
export interface OracleMsg {
    contractAddress: string;
    sender: string;
    acceptAdminRole: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updateConfig: ({ admin, scheduler, worker }: {
        admin?: string;
        scheduler?: string;
        worker?: string;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updatePrices: ({ data }: {
        data: RawPriceItem[];
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export declare class OracleMsgComposer implements OracleMsg {
    sender: string;
    contractAddress: string;
    constructor(sender: string, contractAddress: string);
    acceptAdminRole: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updateConfig: ({ admin, scheduler, worker }: {
        admin?: string | undefined;
        scheduler?: string | undefined;
        worker?: string | undefined;
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
    updatePrices: ({ data }: {
        data: RawPriceItem[];
    }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
