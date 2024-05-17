/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
export interface InstantiateMsg {
    scheduler?: string | null;
    worker?: string | null;
}
export type ExecuteMsg = {
    accept_admin_role: {};
} | {
    update_config: {
        admin?: string | null;
        scheduler?: string | null;
        worker?: string | null;
    };
} | {
    update_prices: {
        data: RawPriceItem[];
    };
};
export type Uint128 = string;
export type TokenUnverified = {
    native: {
        denom: string;
    };
} | {
    cw20: {
        address: string;
    };
};
export interface RawPriceItem {
    collection_address: string;
    price: FundsForTokenUnverified;
}
export interface FundsForTokenUnverified {
    amount: Uint128;
    currency: CurrencyForTokenUnverified;
}
export interface CurrencyForTokenUnverified {
    decimals: number;
    token: TokenUnverified;
}
export type QueryMsg = {
    query_config: {};
} | {
    query_prices: {
        collection_addresses?: string[] | null;
        limit?: number | null;
        start_after?: string | null;
    };
} | {
    query_block_time: {};
};
export interface MigrateMsg {
    version: string;
}
export type Uint64 = number;
export type Addr = string;
export interface Config {
    admin: Addr;
    scheduler?: Addr | null;
    worker?: Addr | null;
}
export type Token = {
    native: {
        denom: string;
    };
} | {
    cw20: {
        address: Addr;
    };
};
export type ArrayOfPriceItem = PriceItem[];
export interface PriceItem {
    address: Addr;
    price: FundsForToken;
    price_update_date: number;
}
export interface FundsForToken {
    amount: Uint128;
    currency: CurrencyForToken;
}
export interface CurrencyForToken {
    decimals: number;
    token: Token;
}
