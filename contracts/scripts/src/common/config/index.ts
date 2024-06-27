import { ChainConfig } from "../../common/interfaces";
import { $, toJson } from "./config-utils";
import * as TreasuryTypes from "../codegen/Treasury.types";
import * as PlatformTypes from "../codegen/Platform.types";

export type NetworkName = "STARGAZE";

export type Wasm = "treasury.wasm" | "platform.wasm";

export const ADDRESS = {
  TESTNET: {
    ADMIN: "stars1f37v0rdvrred27tlqqcpkrqpzfv6ddr2a97zzu",
    WORKER: "stars1hvp3q00ypzrurd46h7c7c3hu86tx9uf8sg5lm3",
  },
  MAINNET: {
    ADMIN: "stars1f37v0rdvrred27tlqqcpkrqpzfv6ddr2a97zzu",
    WORKER: "stars1hvp3q00ypzrurd46h7c7c3hu86tx9uf8sg5lm3",
  },
};

/**
 * This config is used to generate `config.json` used by any script (ts, js, bash).
 * It must be filled manually. If any contract must be added it's required to include
 * it with default parameters - code is 0, address is "".
 * This config uses logs.json generated by local-interchaintest to update endpoints
 * in cofig.json.
 */
export const CHAIN_CONFIG: ChainConfig = {
  CHAINS: [
    {
      NAME: "stargaze",
      PREFIX: "stars",
      OPTIONS: [
        {
          TYPE: "test",
          DENOM: "ustars",
          CHAIN_ID: "elgafar-1",
          RPC_LIST: ["https://rpc.elgafar-1.stargaze-apis.com:443"],
          GAS_PRICE_AMOUNT: 0.04,
          STORE_CODE_GAS_MULTIPLIER: 22,
          CONTRACTS: [
            {
              WASM: "platform.wasm",
              LABEL: "platform",
              INIT_MSG: toJson({}),
              MIGRATE_MSG: toJson<PlatformTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<PlatformTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 4198,
              ADDRESS: "",
            },

            {
              WASM: "treasury.wasm",
              LABEL: "treasury",
              INIT_MSG: toJson<TreasuryTypes.InstantiateMsg>({
                worker: ADDRESS.TESTNET.WORKER,
                platform_code_id: $(
                  "OPTIONS[CHAIN_ID=elgafar-1]|CONTRACTS[WASM=platform.wasm]|CODE"
                ),
              }),
              MIGRATE_MSG: toJson<TreasuryTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<TreasuryTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 4199,
              ADDRESS:
                "stars1e9kkg852sz22703v32dgn2fnhau3sffzrk0adg0zks9nue3huclqy8qrz9",
            },
          ],
          IBC: [],
        },
        {
          TYPE: "main",
          DENOM: "ustars",
          CHAIN_ID: "stargaze-1",
          RPC_LIST: ["https://stargaze-rpc.reece.sh:443"],
          GAS_PRICE_AMOUNT: 1.1,
          STORE_CODE_GAS_MULTIPLIER: 19.5,
          CONTRACTS: [
            {
              WASM: "platform.wasm",
              LABEL: "platform",
              INIT_MSG: toJson({}),
              MIGRATE_MSG: toJson<PlatformTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<PlatformTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 281,
              ADDRESS: "",
            },

            {
              WASM: "treasury.wasm",
              LABEL: "treasury",
              INIT_MSG: toJson<TreasuryTypes.InstantiateMsg>({
                worker: ADDRESS.MAINNET.WORKER,
                platform_code_id: $(
                  "OPTIONS[CHAIN_ID=stargaze-1]|CONTRACTS[WASM=platform.wasm]|CODE"
                ),
              }),
              MIGRATE_MSG: toJson<TreasuryTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<TreasuryTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 282,
              ADDRESS: "",
            },
          ],
          IBC: [],
        },
      ],
    },
  ],
};
