import { ChainConfig } from "../../common/interfaces";
import { $, toJson } from "./config-utils";
import * as PlatformTypes from "../codegen/Platform.types";

export type NetworkName = "STARGAZE";

export type Wasm = "platform.wasm";

export const ADDRESS = {
  ADMIN: "stars1f37v0rdvrred27tlqqcpkrqpzfv6ddr2a97zzu",
  WORKER: "stars1hvp3q00ypzrurd46h7c7c3hu86tx9uf8sg5lm3",
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
              INIT_MSG: toJson<PlatformTypes.InstantiateMsg>({
                worker: ADDRESS.WORKER,
                box_price: `${100_000_000}`,
                distribution: [
                  { box_rewards: `${0}`, weight: "0.282465" },
                  { box_rewards: `${50_000_000}`, weight: "0.3995" },
                  { box_rewards: `${150_000_000}`, weight: "0.13316" },
                  { box_rewards: `${200_000_000}`, weight: "0.099875" },
                  { box_rewards: `${250_000_000}`, weight: "0.0799" },
                  { box_rewards: `${1_000_000_000}`, weight: "0.0051" },
                ],
              }),
              MIGRATE_MSG: toJson<PlatformTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<PlatformTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 0,
              ADDRESS: "",
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
              INIT_MSG: toJson<PlatformTypes.InstantiateMsg>({}),
              MIGRATE_MSG: toJson<PlatformTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson({}),
              CODE: 0,
              ADDRESS: "",
            },
          ],
          IBC: [],
        },
      ],
    },
  ],
};
