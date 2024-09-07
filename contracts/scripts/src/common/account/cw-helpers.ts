import CONFIG_JSON from "../config/config.json";
import { getLast, l, logAndReturn } from "../utils";
import { toBase64, fromUtf8 } from "@cosmjs/encoding";
import { TreasuryMsgComposer } from "../codegen/Treasury.message-composer";
import { TreasuryQueryClient } from "../codegen/Treasury.client";
import { PlatformMsgComposer } from "../codegen/Platform.message-composer";
import { PlatformQueryClient } from "../codegen/Platform.client";
import { getChainOptionById, getContractByWasm } from "../config/config-utils";
import { WeightInfo } from "../codegen/Platform.types";
import { NftInfoForString } from "../codegen/Treasury.types";
import {
  getCwClient,
  signAndBroadcastWrapper,
  getExecuteContractMsg,
} from "./clients";
import {
  SigningCosmWasmClient,
  CosmWasmClient,
  MsgExecuteContractEncodeObject,
} from "@cosmjs/cosmwasm-stargate";
import {
  DirectSecp256k1HdWallet,
  OfflineSigner,
  OfflineDirectSigner,
  coin,
} from "@cosmjs/proto-signing";
import {
  Cw20SendMsg,
  TokenUnverified,
  ChainConfig,
  ContractInfo,
  QueryAllOperatorsResponse,
  QueryAllOperatorsMsg,
  ApproveAllMsg,
  RevokeAllMsg,
  QueryApprovalsMsg,
  ApprovalsResponse,
  QueryTokens,
  TokensResponse,
  QueryOwnerOf,
  OwnerOfResponse,
} from "../interfaces";

function addSingleTokenToComposerObj(
  obj: MsgExecuteContractEncodeObject,
  amount: number,
  token: TokenUnverified
): MsgExecuteContractEncodeObject {
  const {
    value: { contract, sender, msg },
  } = obj;

  if (!(contract && sender && msg)) {
    throw new Error(`${msg} parameters error!`);
  }

  return getSingleTokenExecMsg(
    contract,
    sender,
    JSON.parse(fromUtf8(msg)),
    amount,
    token
  );
}

function getSingleTokenExecMsg(
  contractAddress: string,
  senderAddress: string,
  msg: any,
  amount?: number,
  token?: TokenUnverified
) {
  // get msg without funds
  if (!(token && amount)) {
    return getExecuteContractMsg(contractAddress, senderAddress, msg, []);
  }

  // get msg with native token
  if ("native" in token) {
    return getExecuteContractMsg(contractAddress, senderAddress, msg, [
      coin(amount, token.native.denom),
    ]);
  }

  // get msg with CW20 token
  const cw20SendMsg: Cw20SendMsg = {
    send: {
      contract: contractAddress,
      amount: `${amount}`,
      msg: toBase64(msg),
    },
  };

  return getExecuteContractMsg(
    token.cw20.address,
    senderAddress,
    cw20SendMsg,
    []
  );
}

function getApproveCollectionMsg(
  collectionAddress: string,
  senderAddress: string,
  operator: string
): MsgExecuteContractEncodeObject {
  const approveAllMsg: ApproveAllMsg = {
    approve_all: {
      operator,
    },
  };

  return getSingleTokenExecMsg(collectionAddress, senderAddress, approveAllMsg);
}

function getRevokeCollectionMsg(
  collectionAddress: string,
  senderAddress: string,
  operator: string
): MsgExecuteContractEncodeObject {
  const revokeAllMsg: RevokeAllMsg = {
    revoke_all: {
      operator,
    },
  };

  return getSingleTokenExecMsg(collectionAddress, senderAddress, revokeAllMsg);
}

async function getCwExecHelpers(
  chainId: string,
  rpc: string,
  owner: string,
  signer: (OfflineSigner & OfflineDirectSigner) | DirectSecp256k1HdWallet,
  contractAddress?: string
) {
  const CHAIN_CONFIG = CONFIG_JSON as ChainConfig;
  const {
    OPTION: { CONTRACTS },
  } = getChainOptionById(CHAIN_CONFIG, chainId);

  let TREASURY_CONTRACT: ContractInfo | undefined;
  let PLATFORM_CONTRACT: ContractInfo | undefined;

  try {
    TREASURY_CONTRACT = getContractByWasm(CONTRACTS, "treasury.wasm");
  } catch (error) {
    l(error);
  }

  try {
    PLATFORM_CONTRACT = getContractByWasm(CONTRACTS, "platform.wasm");
  } catch (error) {
    l(error);
  }

  const cwClient = await getCwClient(rpc, owner, signer);
  if (!cwClient) throw new Error("cwClient is not found!");

  const signingClient = cwClient.client as SigningCosmWasmClient;
  const _signAndBroadcast = signAndBroadcastWrapper(signingClient, owner);

  const treasuryMsgComposer = new TreasuryMsgComposer(
    owner,
    TREASURY_CONTRACT ? TREASURY_CONTRACT.ADDRESS : ""
  );

  const platformMsgComposer = new PlatformMsgComposer(
    owner,
    contractAddress || (PLATFORM_CONTRACT ? PLATFORM_CONTRACT.ADDRESS : "")
  );

  async function _msgWrapperWithGasPrice(
    msgs: MsgExecuteContractEncodeObject[],
    gasPrice: string,
    gasAdjustment: number = 1,
    memo?: string
  ) {
    const tx = await _signAndBroadcast(msgs, gasPrice, gasAdjustment, memo);
    l("\n", tx, "\n");
    return tx;
  }

  // utils

  async function cwRevoke(
    collectionAddress: string,
    senderAddress: string,
    operator: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [getRevokeCollectionMsg(collectionAddress, senderAddress, operator)],
      gasPrice
    );
  }

  // treasury

  async function cwIncreaseBalance(
    amount: number,
    denom: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        addSingleTokenToComposerObj(
          treasuryMsgComposer.increaseBalance(),
          amount,
          {
            native: { denom },
          }
        ),
      ],
      gasPrice
    );
  }

  async function cwTreasurySend(
    payment: number,
    rewards: number,
    denom: string,
    recipient: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        treasuryMsgComposer.send({
          payment: payment.toString(),
          rewards: rewards.toString(),
          denom,
          recipient,
        }),
      ],
      gasPrice
    );
  }

  async function cwIncreaseRewards(
    amount: number,
    denom: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        treasuryMsgComposer.increaseRewards({
          amount: amount.toString(),
          denom,
        }),
      ],
      gasPrice
    );
  }

  async function cwSendNft(
    collection: string,
    tokenId: string,
    recipient: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        treasuryMsgComposer.sendNft({
          collection,
          tokenId,
          recipient,
        }),
      ],
      gasPrice
    );
  }

  async function cwTreasuryAcceptAdminRole(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.acceptAdminRole()],
      gasPrice
    );
  }

  async function cwCreatePlatform(
    {
      boxPrice,
      denom,
      distribution,
    }: {
      boxPrice: number;
      denom: string;
      distribution?: WeightInfo[];
    },
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        treasuryMsgComposer.createPlatform({
          boxPrice: boxPrice.toString(),
          denom,
          distribution,
        }),
      ],
      gasPrice
    );
  }

  async function cwAddPlatform(address: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.addPlatform({ address })],
      gasPrice
    );
  }

  async function cwRemovePlatform(address: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.removePlatform({ address })],
      gasPrice
    );
  }

  async function cwDeposit(amount: number, denom: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [
        addSingleTokenToComposerObj(treasuryMsgComposer.deposit(), amount, {
          native: { denom },
        }),
      ],
      gasPrice
    );
  }

  async function cwApproveAndDepositNft(
    senderAddress: string,
    operator: string,
    nftInfoList: NftInfoForString[],
    gasPrice: string
  ) {
    const collectionList = Array.from(
      new Set(nftInfoList.map((x) => x.collection))
    );
    const queryAllOperatorsMsg: QueryAllOperatorsMsg = {
      all_operators: {
        owner: senderAddress,
      },
    };

    let msgList: MsgExecuteContractEncodeObject[] = [];

    for (const collectionAddress of collectionList) {
      const { operators }: QueryAllOperatorsResponse =
        await signingClient.queryContractSmart(
          collectionAddress,
          queryAllOperatorsMsg
        );

      const targetOperator = operators.find((x) => x.spender === operator);

      if (!targetOperator) {
        msgList.push(
          getApproveCollectionMsg(collectionAddress, senderAddress, operator)
        );
      }
    }

    msgList.push(treasuryMsgComposer.depositNft({ nftInfoList }));

    return await _msgWrapperWithGasPrice(msgList, gasPrice);
  }

  async function cwTreasuryUpdateConfig(
    {
      admin,
      worker,
      platformCodeId,
    }: {
      admin?: string;
      worker?: string;
      platformCodeId?: number;
    },
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        treasuryMsgComposer.updateConfig({
          admin,
          worker,
          platformCodeId,
        }),
      ],
      gasPrice,
      1.1
    );
  }

  async function cwTreasuryLock(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.lock()],
      gasPrice
    );
  }

  async function cwTreasuryUnlock(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.unlock()],
      gasPrice
    );
  }

  async function cwWithdraw(amount: number, denom: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.withdraw({ amount: amount.toString(), denom })],
      gasPrice
    );
  }

  async function cwWithdrawNft(
    nftInfoList: NftInfoForString[],
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.withdrawNft({ nftInfoList })],
      gasPrice
    );
  }

  async function cwUpdateNftPrice(
    nftInfoList: NftInfoForString[],
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [treasuryMsgComposer.updateNftPrice({ nftInfoList })],
      gasPrice
    );
  }

  // platform

  async function cwBuy(amount: number, denom: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [
        addSingleTokenToComposerObj(platformMsgComposer.buy(), amount, {
          native: { denom },
        }),
      ],
      gasPrice
    );
  }

  async function cwOpen(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.open()],
      gasPrice
    );
  }

  async function cwBuyAndOpen(amount: number, denom: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [
        addSingleTokenToComposerObj(platformMsgComposer.buy(), amount, {
          native: { denom },
        }),
        platformMsgComposer.open(),
      ],
      gasPrice
    );
  }

  /** for tests */
  async function cwOpenMultiple(amount: number, gasPrice: string) {
    const msg = platformMsgComposer.open();

    return await _msgWrapperWithGasPrice(
      [...new Array(amount)].map(() => msg),
      gasPrice
    );
  }

  async function cwClaim(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.claim()],
      gasPrice
    );
  }

  async function cwPlatformSend(
    amount: number,
    recipient: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.send({ amount: amount.toString(), recipient })],
      gasPrice
    );
  }

  async function cwPlatformAcceptAdminRole(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.acceptAdminRole()],
      gasPrice
    );
  }

  async function cwPlatformUpdateConfig(
    {
      admin,
      worker,
      boxPrice,
      denom,
      distribution,
    }: {
      admin?: string;
      worker?: string;
      boxPrice?: number;
      denom?: string;
      distribution?: WeightInfo[];
    },
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        platformMsgComposer.updateConfig({
          admin,
          worker,
          boxPrice: boxPrice?.toString(),
          denom,
          distribution,
        }),
      ],
      gasPrice
    );
  }

  async function cwPlatformLock(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.lock()],
      gasPrice
    );
  }

  async function cwPlatformUnlock(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.unlock()],
      gasPrice
    );
  }

  return {
    utils: { cwRevoke },
    treasury: {
      cwIncreaseBalance,
      cwSend: cwTreasurySend,
      cwIncreaseRewards,
      cwSendNft,
      cwAcceptAdminRole: cwTreasuryAcceptAdminRole,
      cwCreatePlatform,
      cwAddPlatform,
      cwRemovePlatform,
      cwDeposit,
      cwApproveAndDepositNft,
      cwUpdateConfig: cwTreasuryUpdateConfig,
      cwLock: cwTreasuryLock,
      cwUnlock: cwTreasuryUnlock,
      cwWithdraw,
      cwWithdrawNft,
      cwUpdateNftPrice,
    },
    platform: {
      cwBuy,
      cwOpen,
      cwBuyAndOpen,
      cwClaim,
      cwSend: cwPlatformSend,
      cwAcceptAdminRole: cwPlatformAcceptAdminRole,
      cwUpdateConfig: cwPlatformUpdateConfig,
      cwLock: cwPlatformLock,
      cwUnlock: cwPlatformUnlock,

      cwOpenMultiple,
    },
  };
}

async function getCwQueryHelpers(
  chainId: string,
  rpc: string,
  contractAddress?: string
) {
  const CHAIN_CONFIG = CONFIG_JSON as ChainConfig;
  const {
    OPTION: { CONTRACTS },
  } = getChainOptionById(CHAIN_CONFIG, chainId);

  let TREASURY_CONTRACT: ContractInfo | undefined;
  let PLATFORM_CONTRACT: ContractInfo | undefined;

  try {
    TREASURY_CONTRACT = getContractByWasm(CONTRACTS, "treasury.wasm");
  } catch (error) {
    l(error);
  }

  try {
    PLATFORM_CONTRACT = getContractByWasm(CONTRACTS, "platform.wasm");
  } catch (error) {
    l(error);
  }

  const cwClient = await getCwClient(rpc);
  if (!cwClient) throw new Error("cwClient is not found!");

  const cosmwasmQueryClient: CosmWasmClient = cwClient.client;

  const treasuryQueryClient = new TreasuryQueryClient(
    cosmwasmQueryClient,
    TREASURY_CONTRACT ? TREASURY_CONTRACT.ADDRESS : ""
  );

  const platformQueryClient = new PlatformQueryClient(
    cosmwasmQueryClient,
    contractAddress || (PLATFORM_CONTRACT ? PLATFORM_CONTRACT.ADDRESS : "")
  );

  // utils

  async function cwQueryOperators(
    ownerAddress: string,
    collectionAddress: string,
    isDisplayed: boolean = false
  ) {
    const queryAllOperatorsMsg: QueryAllOperatorsMsg = {
      all_operators: {
        owner: ownerAddress,
      },
    };
    const res: QueryAllOperatorsResponse =
      await cosmwasmQueryClient.queryContractSmart(
        collectionAddress,
        queryAllOperatorsMsg
      );
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryApprovals(
    collectionAddress: string,
    tokenId: string,
    isDisplayed: boolean = false
  ) {
    const queryApprovalsMsg: QueryApprovalsMsg = {
      approvals: {
        token_id: tokenId,
      },
    };
    const res: ApprovalsResponse = await cosmwasmQueryClient.queryContractSmart(
      collectionAddress,
      queryApprovalsMsg
    );
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryBalanceInNft(
    owner: string,
    collectionAddress: string,
    isDisplayed: boolean = false
  ) {
    const MAX_LIMIT = 100;
    const ITER_LIMIT = 50;

    let tokenList: string[] = [];
    let tokenAmountSum: number = 0;
    let i: number = 0;
    let lastToken: string | undefined = undefined;

    while ((!i || tokenAmountSum === MAX_LIMIT) && i < ITER_LIMIT) {
      i++;

      try {
        const queryTokensMsg: QueryTokens = {
          tokens: {
            owner,
            start_after: lastToken,
            limit: MAX_LIMIT,
          },
        };

        const { tokens }: TokensResponse =
          await cosmwasmQueryClient.queryContractSmart(
            collectionAddress,
            queryTokensMsg
          );

        tokenList = [...tokenList, ...tokens];
        tokenAmountSum = tokens.length;
        lastToken = getLast(tokens);
      } catch (error) {
        l(error);
      }
    }

    const res: TokensResponse = { tokens: tokenList };
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryNftOwner(
    collectionAddress: string,
    tokenId: string,
    isDisplayed: boolean = false
  ) {
    const queryOwnerOfMsg: QueryOwnerOf = {
      owner_of: { token_id: tokenId },
    };
    const res: OwnerOfResponse = await cosmwasmQueryClient.queryContractSmart(
      collectionAddress,
      queryOwnerOfMsg
    );
    return logAndReturn(res, isDisplayed);
  }

  // treasury

  async function cwTreasuryQueryConfig(isDisplayed: boolean = false) {
    const res = await treasuryQueryClient.queryConfig();
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryBalance(isDisplayed: boolean = false) {
    const res = await treasuryQueryClient.queryBalance();
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryPlatformList(isDisplayed: boolean = false) {
    const res = await treasuryQueryClient.queryPlatformList();
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryRemovedPlatformList(isDisplayed: boolean = false) {
    const res = await treasuryQueryClient.queryRemovedPlatformList();
    return logAndReturn(res, isDisplayed);
  }

  // platform

  async function cwPlatformQueryConfig(isDisplayed: boolean = false) {
    const res = await platformQueryClient.queryConfig();
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryBoxStats(isDisplayed: boolean = false) {
    const res = await platformQueryClient.queryBoxStats();
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryUser(address: string, isDisplayed: boolean = false) {
    const res = await platformQueryClient.queryUser({ address });
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryUserList(
    limit: number = 10_000,
    startAfter: string | undefined = undefined,
    isDisplayed: boolean = false
  ) {
    const res = await platformQueryClient.queryUserList({ startAfter, limit });
    return logAndReturn(res, isDisplayed);
  }

  return {
    utils: {
      cwQueryOperators,
      cwQueryApprovals,
      cwQueryBalanceInNft,
      cwQueryNftOwner,
    },
    treasury: {
      cwQueryConfig: cwTreasuryQueryConfig,
      cwQueryBalance,
      cwQueryPlatformList,
      cwQueryRemovedPlatformList,
    },
    platfrorm: {
      cwQueryConfig: cwPlatformQueryConfig,
      cwQueryBoxStats,
      cwQueryUser,
      cwQueryUserList,
    },
  };
}

export { getCwExecHelpers, getCwQueryHelpers };
