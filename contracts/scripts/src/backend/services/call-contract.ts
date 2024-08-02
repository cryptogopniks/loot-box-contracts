import { getSigner } from "../account/signer";
import { getLast, l, li, wait } from "../../common/utils";
import { readFile } from "fs/promises";
import { ChainConfig } from "../../common/interfaces";
import { ADDRESS } from "../../common/config";
import { coin } from "@cosmjs/stargate";
import {
  ENCODING,
  PATH_TO_CONFIG_JSON,
  getWallets,
  parseStoreArgs,
  parseWasmAttribute,
} from "./utils";
import {
  getChainOptionById,
  getContractByWasm,
} from "../../common/config/config-utils";
import {
  getSgQueryHelpers,
  getSgExecHelpers,
} from "../../common/account/sg-helpers";
import {
  getCwExecHelpers,
  getCwQueryHelpers,
} from "../../common/account/cw-helpers";

async function main() {
  try {
    const configJsonStr = await readFile(PATH_TO_CONFIG_JSON, {
      encoding: ENCODING,
    });
    const CHAIN_CONFIG: ChainConfig = JSON.parse(configJsonStr);
    const { chainId } = parseStoreArgs();
    const {
      PREFIX: PREFIX,
      OPTION: {
        RPC_LIST: [RPC],
        DENOM: DENOM,
        CONTRACTS: CONTRACTS,
        GAS_PRICE_AMOUNT: GAS_PRICE_AMOUNT,
        TYPE: TYPE,
      },
    } = getChainOptionById(CHAIN_CONFIG, chainId);

    const PLATFORM_CONTRACT = getContractByWasm(CONTRACTS, "platform.wasm");
    const gasPrice = `${GAS_PRICE_AMOUNT}${DENOM}`;
    const testWallets = await getWallets(TYPE);
    const { signer, owner } = await getSigner(PREFIX, testWallets.SEED_ADMIN);

    const sgQueryHelpers = await getSgQueryHelpers(RPC);
    const sgExecHelpers = await getSgExecHelpers(RPC, owner, signer);

    const { utils, treasury } = await getCwQueryHelpers(chainId, RPC);
    const h = await getCwExecHelpers(chainId, RPC, owner, signer);

    const { getBalance, getAllBalances } = sgQueryHelpers;
    const { sgMultiSend, sgSend } = sgExecHelpers;

    await treasury.cwQueryConfig();

    // await h.treasury.cwCreatePlatform(
    //   {
    //     boxPrice: 100_000_000,
    //     denom: DENOM,
    //     distribution: [
    //       { box_rewards: `${0}`, weight: "0.282465" },
    //       { box_rewards: `${50_000_000}`, weight: "0.3995" },
    //       { box_rewards: `${150_000_000}`, weight: "0.13316" },
    //       { box_rewards: `${200_000_000}`, weight: "0.099875" },
    //       { box_rewards: `${250_000_000}`, weight: "0.0799" },
    //       { box_rewards: `${1_000_000_000}`, weight: "0.0051" },
    //     ],
    //   },
    //   gasPrice
    // );
    // await h.treasury.cwCreatePlatform(
    //   {
    //     boxPrice: 200_000_000,
    //     denom: DENOM,
    //     distribution: [
    //       { box_rewards: `${0}`, weight: "0.282465" },
    //       { box_rewards: `${100_000_000}`, weight: "0.3995" },
    //       { box_rewards: `${300_000_000}`, weight: "0.13316" },
    //       { box_rewards: `${400_000_000}`, weight: "0.099875" },
    //       { box_rewards: `${500_000_000}`, weight: "0.0799" },
    //       { box_rewards: `${2_000_000_000}`, weight: "0.0051" },
    //     ],
    //   },
    //   gasPrice
    // );

    // await h.treasury.cwCreatePlatform(
    //   {
    //     boxPrice: 10_000_000,
    //     denom: DENOM,
    //     distribution: [
    //       { box_rewards: `${0}`, weight: "0.575" },
    //       { box_rewards: `${20_000_000}`, weight: "0.425" },
    //     ],
    //   },
    //   gasPrice
    // );

    await treasury.cwQueryBalance(true);
    const platformList = await treasury.cwQueryPlatformList(true);
    const platformAddress = getLast(platformList);
    const platformConfig = await (
      await getCwQueryHelpers(chainId, RPC, platformAddress)
    ).platfrorm.cwQueryConfig(true);

    // await h.treasury.cwRemovePlatform(platformAddress, gasPrice);
    // await treasury.cwQueryRemovedPlatformList();
  } catch (error) {
    l(error);
  }
}

main();
