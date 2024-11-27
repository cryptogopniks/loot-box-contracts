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

    const TREASURY_CONTRACT = getContractByWasm(CONTRACTS, "treasury.wasm");
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

    const joe = "archway1fxvjt83z94sny0e9cy23sr43r6whz6c9qczn3d";
    const gop = "archway1hvp3q00ypzrurd46h7c7c3hu86tx9uf83llx6h";
    // await h.treasury.cwUpdateConfig({ worker: joe }, gasPrice);

    const COLLECTION =
      "stars1h4lzfpcyr38yj9mysnzqy8360tt3uggmdfukgrutgmck53rn9r9sj2qg9q";
    const ID_LIST = ["1729", "9832"];

    await h.treasury.cwRetractNft(
      [
        {
          collection: COLLECTION,
          token_id: ID_LIST[0],
          price_option: [],
        },
        {
          collection: COLLECTION,
          token_id: ID_LIST[1],
          price_option: [],
        },
      ],
      gasPrice
    );

    // await utils.cwQueryBalanceInNft(owner, COLLECTION, true);
    const nftList = (await treasury.cwQueryBalance()).nft_pool;
    const targetCollection = nftList.find((x) => x.collection === COLLECTION);
    li({ targetCollection });

    return;

    // await utils.cwQueryNftOwner(COLLECTION, ID_LIST[0], true);
    // await utils.cwQueryNftOwner(COLLECTION, ID_LIST[1], true);
    // await treasury.cwQueryConfig(true);
    // await treasury.cwQueryPlatformList(true);

    // for (const platformAddress of PLATFORM_LIST) {
    //   l(platformAddress);
    //   const queryHelpers = await getCwQueryHelpers(
    //     chainId,
    //     RPC,
    //     platformAddress
    //   );
    //   await queryHelpers.platfrorm.cwQueryConfig(true);
    //   await queryHelpers.platfrorm.cwQueryBoxStats(true);
    // }

    // await utils.cwQueryBalanceInNft(
    //   TREASURY_CONTRACT.ADDRESS,
    //   COLLECTION,
    //   true
    // );

    // for (let i = 0; i < 2; i++) {
    //   await h.treasury.cwCreatePlatform(
    //     {
    //       boxPrice: 100,
    //       denom: DENOM,
    //       distribution: [
    //         { box_rewards: `${0}`, weight: "0.54" },
    //         { box_rewards: `${200}`, weight: "0.46" },
    //       ],
    //     },
    //     gasPrice
    //   );
    // }
    // return;

    await treasury.cwQueryConfig(true);
    await treasury.cwQueryBalance(true);
    const platformList = await treasury.cwQueryPlatformList(true);
    // return;

    for (const platformAddress of platformList) {
      l(platformAddress);
      const queryHelpers = await getCwQueryHelpers(
        chainId,
        RPC,
        platformAddress
      );
      await queryHelpers.platfrorm.cwQueryConfig(true);
      await queryHelpers.platfrorm.cwQueryBoxStats(true);
    }
    return;

    const platformAddress = platformList[0];
    const p = await getCwExecHelpers(
      chainId,
      RPC,
      owner,
      signer,
      platformAddress
    );
    const q = await getCwQueryHelpers(chainId, RPC, platformAddress);

    await p.platform.cwBuyAndOpen(100, DENOM, gasPrice);
    // await p.platform.cwBuy(100, "aarch", gasPrice);
    // await p.platform.cwOpen(gasPrice);

    // const gop = "archway1hvp3q00ypzrurd46h7c7c3hu86tx9uf83llx6h";
    // await q.platfrorm.cwQueryUser(gop, true);

    return;

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

    await h.treasury.cwCreatePlatform(
      {
        boxPrice: 100,
        denom: DENOM,
        distribution: [
          { box_rewards: `${0}`, weight: "0.55" },
          { box_rewards: `${200}`, weight: "0.45" },
        ],
      },
      gasPrice
    );

    // await treasury.cwQueryBalance(true);
    // const platformList = await treasury.cwQueryPlatformList(true);
    // const platformAddress = getLast(platformList);
    // const platformConfig = await (
    //   await getCwQueryHelpers(chainId, RPC, platformAddress)
    // ).platfrorm.cwQueryConfig(true);

    // await h.treasury.cwRemovePlatform(platformAddress, gasPrice);
    // await treasury.cwQueryRemovedPlatformList();
  } catch (error) {
    l(error);
  }
}

main();
