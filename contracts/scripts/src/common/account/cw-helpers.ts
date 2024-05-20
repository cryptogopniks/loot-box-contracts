import CONFIG_JSON from "../config/config.json";
import { getLast, l, logAndReturn } from "../utils";
import { toBase64, fromUtf8 } from "@cosmjs/encoding";
import { PlatformMsgComposer } from "../codegen/Platform.message-composer";
import { PlatformQueryClient } from "../codegen/Platform.client";
import { getChainOptionById, getContractByWasm } from "../config/config-utils";
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
import { NftInfoForString, WeightInfo } from "../codegen/Platform.types";

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

  let PLATFORM_CONTRACT: ContractInfo | undefined;

  try {
    PLATFORM_CONTRACT = getContractByWasm(CONTRACTS, "platform.wasm");
  } catch (error) {
    l(error);
  }

  const cwClient = await getCwClient(rpc, owner, signer);
  if (!cwClient) throw new Error("cwClient is not found!");

  const signingClient = cwClient.client as SigningCosmWasmClient;
  const _signAndBroadcast = signAndBroadcastWrapper(signingClient, owner);

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

  async function cwSend(amount: number, recipient: string, gasPrice: string) {
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

  async function cwDeposit(amount: number, denom: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [
        addSingleTokenToComposerObj(platformMsgComposer.deposit(), amount, {
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

    msgList.push(platformMsgComposer.depositNft({ nftInfoList }));

    return await _msgWrapperWithGasPrice(msgList, gasPrice);
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

  async function cwLock(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.lock()],
      gasPrice
    );
  }

  async function cwUnlock(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.unlock()],
      gasPrice
    );
  }

  async function cwWithdraw(amount: number, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.withdraw({ amount: amount.toString() })],
      gasPrice
    );
  }

  async function cwWithdrawNft(
    nftInfoList: NftInfoForString[],
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.withdrawNft({ nftInfoList })],
      gasPrice
    );
  }

  async function cwUpdateNftPrice(
    nftInfoList: NftInfoForString[],
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [platformMsgComposer.updateNftPrice({ nftInfoList })],
      gasPrice
    );
  }

  return {
    utils: { cwRevoke },
    platform: {
      cwBuy,
      cwOpen,
      cwClaim,
      cwSend,
      cwAcceptAdminRole: cwPlatformAcceptAdminRole,
      cwDeposit,
      cwApproveAndDepositNft,
      cwUpdateConfig: cwPlatformUpdateConfig,
      cwLock,
      cwUnlock,
      cwWithdraw,
      cwWithdrawNft,
      cwUpdateNftPrice,

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

  let PLATFORM_CONTRACT: ContractInfo | undefined;

  try {
    PLATFORM_CONTRACT = getContractByWasm(CONTRACTS, "platform.wasm");
  } catch (error) {
    l(error);
  }

  const cwClient = await getCwClient(rpc);
  if (!cwClient) throw new Error("cwClient is not found!");

  const cosmwasmQueryClient: CosmWasmClient = cwClient.client;

  const platformQueryClient = new PlatformQueryClient(
    cosmwasmQueryClient,
    contractAddress || (PLATFORM_CONTRACT ? PLATFORM_CONTRACT.ADDRESS : "")
  );

  // utils

  async function cwQueryOperators(
    ownerAddress: string,
    collectionAddress: string
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
    return logAndReturn(res);
  }

  async function cwQueryApprovals(collectionAddress: string, tokenId: string) {
    const queryApprovalsMsg: QueryApprovalsMsg = {
      approvals: {
        token_id: tokenId,
      },
    };
    const res: ApprovalsResponse = await cosmwasmQueryClient.queryContractSmart(
      collectionAddress,
      queryApprovalsMsg
    );
    return logAndReturn(res);
  }

  async function cwQueryBalanceInNft(owner: string, collectionAddress: string) {
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
    return logAndReturn(res);
  }

  async function cwQueryNftOwner(collectionAddress: string, tokenId: string) {
    const queryOwnerOfMsg: QueryOwnerOf = {
      owner_of: { token_id: tokenId },
    };
    const res: OwnerOfResponse = await cosmwasmQueryClient.queryContractSmart(
      collectionAddress,
      queryOwnerOfMsg
    );
    return logAndReturn(res);
  }

  // platform

  async function cwPlatformQueryConfig() {
    const res = await platformQueryClient.queryConfig();
    return logAndReturn(res);
  }

  async function cwQueryBoxStats() {
    const res = await platformQueryClient.queryBoxStats();
    return logAndReturn(res);
  }

  async function cwQueryBalance() {
    const res = await platformQueryClient.queryBalance();
    return logAndReturn(res);
  }

  async function cwQueryUser(address: string) {
    const res = await platformQueryClient.queryUser({ address });
    return logAndReturn(res);
  }

  async function cwQueryUserList(limit: number = 10_000, startAfter?: string) {
    const res = await platformQueryClient.queryUserList({ startAfter, limit });
    return logAndReturn(res);
  }

  return {
    utils: {
      cwQueryOperators,
      cwQueryApprovals,
      cwQueryBalanceInNft,
      cwQueryNftOwner,
    },
    platfrorm: {
      cwQueryConfig: cwPlatformQueryConfig,
      cwQueryBoxStats,
      cwQueryBalance,
      cwQueryUser,
      cwQueryUserList,
    },
  };
}

export { getCwExecHelpers, getCwQueryHelpers };
