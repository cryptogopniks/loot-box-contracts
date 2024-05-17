import { getSigner } from "../account/signer";
import { l, li, wait } from "../../common/utils";
import { readFile } from "fs/promises";
import { ChainConfig } from "../../common/interfaces";
import { ADDRESS } from "../../common/config";
import {
  ENCODING,
  PATH_TO_CONFIG_JSON,
  getWallets,
  parseStoreArgs,
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

    const { getAllBalances } = sgQueryHelpers;
    const { sgMultiSend } = sgExecHelpers;

    // update config
    const config = await platfrorm.cwQueryConfig();

    if (config.box_list_length !== 400) {
      await h.platform.cwUpdateConfig(
        {
          boxPrice: 100_000_000,
          priceAndWeightList: [
            [0, 0.3925],
            [50, 0.45],
            [250, 0.09],
            [500, 0.045],
            [1000, 0.0225],
          ],
          boxListLength: 400,
        },
        gasPrice
      );
    }

    // request box list
    const { update_date: updateDate } = await platfrorm.cwQueryBoxList();
    await h.platform.cwRequestBoxList(
      1_000,
      { native: { denom: DENOM } },
      gasPrice
    );

    // check results
    while (true) {
      const { update_date: updateDateCurrent, price_list: priceList } =
        await platfrorm.cwQueryBoxList();
      l({ updateDate: updateDateCurrent });

      if (updateDateCurrent !== updateDate) {
        l(priceList);
        break;
      }

      await wait(5_000);
    }
  } catch (error) {
    l(error);
  }
}

main();
