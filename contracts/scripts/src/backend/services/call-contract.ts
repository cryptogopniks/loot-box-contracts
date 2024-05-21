import { getSigner } from "../account/signer";
import { l, li, wait } from "../../common/utils";
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

    const { utils, platfrorm } = await getCwQueryHelpers(chainId, RPC);
    const h = await getCwExecHelpers(chainId, RPC, owner, signer);

    const { getBalance, getAllBalances } = sgQueryHelpers;
    const { sgMultiSend, sgSend } = sgExecHelpers;

    // replenish balance
    const { amount: balance } = await getBalance(
      PLATFORM_CONTRACT.ADDRESS,
      DENOM
    );
    if (Number(balance) < 500_000) {
      await sgSend(PLATFORM_CONTRACT.ADDRESS, coin(1_000_000, DENOM), gasPrice);
    }

    // await platfrorm.cwQueryUser(ADDRESS.ADMIN);
    // await h.platform.cwBuy(1_000, DENOM, gasPrice);
    // await platfrorm.cwQueryUser(ADDRESS.ADMIN);

    // const res = await h.platform.cwOpen(gasPrice);
    // const rewards = parseWasmAttribute(res, "coins");
    // l({ rewards });
    // await platfrorm.cwQueryUser(ADDRESS.ADMIN);

    // await platfrorm.cwQueryUser(ADDRESS.ADMIN);
    // try {
    //   await h.platform.cwOpenMultiple(1, gasPrice);
    // } catch (error) {
    //   l(error);
    // }
    // await platfrorm.cwQueryUser(ADDRESS.ADMIN);

    // await h.platform.cwBuy(1_000, DENOM, gasPrice);
    for (let i = 0; i < 10; i++) {
      const res = await h.platform.cwOpen(gasPrice);
      const rewards = parseWasmAttribute(res, "coins");
      l({ rewards });
      await wait(5_000);
    }
  } catch (error) {
    l(error);
  }
}

main();
