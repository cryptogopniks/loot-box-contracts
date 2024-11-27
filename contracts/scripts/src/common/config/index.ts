import { ChainConfig } from "../../common/interfaces";
import { $, toJson } from "./config-utils";
import * as TreasuryTypes from "../codegen/Treasury.types";
import * as PlatformTypes from "../codegen/Platform.types";

export type NetworkName = "STARGAZE" | "ARCHWAY";

export type Wasm = "treasury.wasm" | "platform.wasm";

export const ADDRESS = {
  STARGAZE: {
    TESTNET: {
      ADMIN: "stars1f37v0rdvrred27tlqqcpkrqpzfv6ddr2a97zzu",
      WORKER: "stars1hvp3q00ypzrurd46h7c7c3hu86tx9uf8sg5lm3",
    },
    MAINNET: {
      ADMIN: "stars1f37v0rdvrred27tlqqcpkrqpzfv6ddr2a97zzu",
      WORKER: "stars1hvp3q00ypzrurd46h7c7c3hu86tx9uf8sg5lm3",
    },
  },

  ARCHWAY: {
    TESTNET: {
      ADMIN: "archway1f37v0rdvrred27tlqqcpkrqpzfv6ddr2uj4mr6",
      WORKER: "archway1hvp3q00ypzrurd46h7c7c3hu86tx9uf83llx6h",
    },
    MAINNET: {
      ADMIN: "archway1f37v0rdvrred27tlqqcpkrqpzfv6ddr2uj4mr6",
      WORKER: "archway1hvp3q00ypzrurd46h7c7c3hu86tx9uf83llx6h",
    },
  },

  CHIHUAHUA: {
    TESTNET: {
      ADMIN: "chihuahua1f37v0rdvrred27tlqqcpkrqpzfv6ddr22vy3g0",
      WORKER: "chihuahua1hvp3q00ypzrurd46h7c7c3hu86tx9uf88pwv3z",
    },
    MAINNET: {
      ADMIN: "chihuahua1f37v0rdvrred27tlqqcpkrqpzfv6ddr22vy3g0",
      WORKER: "chihuahua1hvp3q00ypzrurd46h7c7c3hu86tx9uf88pwv3z",
    },
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
                worker: ADDRESS.STARGAZE.TESTNET.WORKER,
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
          RPC_LIST: ["https://stargaze-rpc.polkachu.com:443"],
          GAS_PRICE_AMOUNT: 1.1,
          STORE_CODE_GAS_MULTIPLIER: 22.5,
          CONTRACTS: [
            {
              WASM: "platform.wasm",
              LABEL: "platform",
              PERMISSION: [
                ADDRESS.STARGAZE.MAINNET.ADMIN,
                ADDRESS.STARGAZE.MAINNET.WORKER,
                "stars1ev6skugnl2lgsj3ts6y8j4563j7qs4vtlvgqgfcwqum840xzu7tqythx58",
              ],
              INIT_MSG: toJson({}),
              MIGRATE_MSG: toJson<PlatformTypes.MigrateMsg>({
                version: "1.1.0",
              }),
              UPDATE_MSG: toJson<PlatformTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 330,
              ADDRESS: "",
            },

            {
              WASM: "treasury.wasm",
              LABEL: "treasury",
              PERMISSION: [
                ADDRESS.STARGAZE.MAINNET.ADMIN,
                ADDRESS.STARGAZE.MAINNET.WORKER,
              ],
              INIT_MSG: toJson<TreasuryTypes.InstantiateMsg>({
                worker: ADDRESS.STARGAZE.MAINNET.WORKER,
                platform_code_id: $(
                  "OPTIONS[CHAIN_ID=stargaze-1]|CONTRACTS[WASM=platform.wasm]|CODE"
                ),
              }),
              MIGRATE_MSG: toJson<TreasuryTypes.MigrateMsg>({
                version: "1.1.0",
              }),
              UPDATE_MSG: toJson<TreasuryTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 469,
              ADDRESS:
                "stars1ev6skugnl2lgsj3ts6y8j4563j7qs4vtlvgqgfcwqum840xzu7tqythx58",
            },
          ],
          IBC: [],
        },
      ],
    },

    {
      NAME: "archway",
      PREFIX: "archway",
      OPTIONS: [
        {
          TYPE: "test",
          DENOM: "aconst",
          CHAIN_ID: "constantine-3",
          RPC_LIST: ["https://rpc.constantine.archway.io:443"],
          GAS_PRICE_AMOUNT: 1500000000000,
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
              CODE: 3216,
              ADDRESS: "",
            },

            {
              WASM: "treasury.wasm",
              LABEL: "treasury",
              INIT_MSG: toJson<TreasuryTypes.InstantiateMsg>({
                worker: ADDRESS.ARCHWAY.TESTNET.WORKER,
                platform_code_id: 3216,
              }),
              MIGRATE_MSG: toJson<TreasuryTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<TreasuryTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 3217,
              ADDRESS:
                "archway15yta6v4k3p4zatcw8nx3hg34wwj495lejhyvswuhwzq90x0s3tasywu0kr",
            },
          ],
          IBC: [],
        },

        {
          TYPE: "main",
          DENOM: "aarch",
          CHAIN_ID: "archway-1",
          RPC_LIST: ["https://m-archway.rpc.utsa.tech:443"],
          GAS_PRICE_AMOUNT: 1500000000000,
          STORE_CODE_GAS_MULTIPLIER: 22,
          CONTRACTS: [
            {
              WASM: "platform.wasm",
              LABEL: "platform",
              INIT_MSG: toJson({}),
              MIGRATE_MSG: toJson<PlatformTypes.MigrateMsg>({
                version: "1.1.0",
              }),
              UPDATE_MSG: toJson<PlatformTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 587,
              ADDRESS: "",
            },

            {
              WASM: "treasury.wasm",
              LABEL: "treasury",
              INIT_MSG: toJson<TreasuryTypes.InstantiateMsg>({
                worker: ADDRESS.ARCHWAY.TESTNET.WORKER,
                platform_code_id: $(
                  "OPTIONS[CHAIN_ID=archway-1]|CONTRACTS[WASM=platform.wasm]|CODE"
                ),
              }),
              MIGRATE_MSG: toJson<TreasuryTypes.MigrateMsg>({
                version: "1.1.0",
              }),
              UPDATE_MSG: toJson<TreasuryTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 588,
              ADDRESS:
                "archway1vumem5fchkltp0t5u22ad3k4qm5ne0fxwhvw798tr2qux9y350cqn8jjf4",
            },
          ],
          IBC: [],
        },
      ],
    },

    {
      NAME: "chihuahua",
      PREFIX: "chihuahua",
      OPTIONS: [
        {
          TYPE: "main",
          DENOM: "uhuahua",
          CHAIN_ID: "chihuahua-1",
          RPC_LIST: ["https://chihuahua-rpc.polkachu.com:443"],
          GAS_PRICE_AMOUNT: 1.1,
          STORE_CODE_GAS_MULTIPLIER: 25.5,
          CONTRACTS: [
            {
              WASM: "platform.wasm",
              LABEL: "platform",
              PERMISSION: [
                ADDRESS.CHIHUAHUA.MAINNET.ADMIN,
                ADDRESS.CHIHUAHUA.MAINNET.WORKER,
                "chihuahua1j926delgh6qhxaqyqmz0pmjhepsqh6u66c23n59n2ew2uuxye7tswkse3k",
              ],
              INIT_MSG: toJson({}),
              MIGRATE_MSG: toJson<PlatformTypes.MigrateMsg>({
                version: "1.1.0",
              }),
              UPDATE_MSG: toJson<PlatformTypes.ExecuteMsg>({
                update_config: {},
              }),
              CODE: 629,
              ADDRESS: "",
            },

            {
              WASM: "treasury.wasm",
              LABEL: "treasury",
              PERMISSION: [
                ADDRESS.CHIHUAHUA.MAINNET.ADMIN,
                ADDRESS.CHIHUAHUA.MAINNET.WORKER,
              ],
              INIT_MSG: toJson<TreasuryTypes.InstantiateMsg>({
                worker: ADDRESS.CHIHUAHUA.MAINNET.WORKER,
              }),
              MIGRATE_MSG: toJson<TreasuryTypes.MigrateMsg>({
                version: "1.1.0",
              }),
              UPDATE_MSG: toJson<TreasuryTypes.ExecuteMsg>({
                update_config: {
                  platform_code_id: $(
                    "OPTIONS[CHAIN_ID=chihuahua-1]|CONTRACTS[WASM=platform.wasm]|CODE"
                  ),
                },
              }),
              CODE: 628,
              ADDRESS:
                "chihuahua1j926delgh6qhxaqyqmz0pmjhepsqh6u66c23n59n2ew2uuxye7tswkse3k",
            },
          ],
          IBC: [],
        },
      ],
    },
  ],
};
