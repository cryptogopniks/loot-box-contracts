import { l, li } from "../utils";
import { getSgClient, signAndBroadcastWrapper } from "./clients";
import { Tendermint37Client } from "@cosmjs/tendermint-rpc";
import { longify } from "@cosmjs/stargate/build/queryclient";
import { setupBankExtension, QueryClient, coin, setupStakingExtension } from "@cosmjs/stargate";
async function getSgExecHelpers(rpc, owner, signer) {
  const sgClient = await getSgClient(rpc, owner, signer);
  if (!sgClient) throw new Error("sgClient is not found!");
  const client = sgClient.client;
  const signAndBroadcast = signAndBroadcastWrapper(client, owner);
  async function sgSend(recipient, amount, gasPrice) {
    const msg = {
      typeUrl: "/cosmos.bank.v1beta1.MsgSend",
      value: {
        fromAddress: owner,
        toAddress: recipient,
        amount: [amount]
      }
    };
    const tx = await signAndBroadcast([msg], gasPrice);
    l("\n", tx, "\n");
    return tx;
  }
  async function sgMultiSend(denom, recipientAndAmountList, gasPrice) {
    const msgs = recipientAndAmountList.map(([address, amount]) => ({
      typeUrl: "/cosmos.bank.v1beta1.MsgSend",
      value: {
        fromAddress: owner,
        toAddress: address,
        amount: [coin(amount, denom)]
      }
    }));
    const tx = await signAndBroadcast(msgs, gasPrice);
    l("\n", tx, "\n");
    return tx;
  }
  async function sgDelegate(operatorAddress, amount, denom, gasPrice) {
    const msg = {
      typeUrl: "/cosmos.staking.v1beta1.MsgDelegate",
      value: {
        delegatorAddress: owner,
        validatorAddress: operatorAddress,
        amount: coin(amount, denom)
      }
    };
    const tx = await signAndBroadcast([msg], gasPrice);
    l("\n", tx, "\n");
    return tx;
  }
  async function sgVote(proposalId, voteOption, gasPrice) {
    const msg = {
      typeUrl: "/cosmos.gov.v1beta1.MsgVote",
      value: {
        voter: owner,
        proposalId: longify(proposalId),
        option: voteOption
      }
    };
    const tx = await signAndBroadcast([msg], gasPrice, 1.2);
    l("\n", tx, "\n");
    return tx;
  }
  return {
    sgDelegate,
    sgVote,
    sgSend,
    sgMultiSend
  };
}
async function getSgQueryHelpers(rpc) {
  const tmClient = await Tendermint37Client.connect(rpc);
  const queryClient = QueryClient.withExtensions(tmClient);
  const bankExtension = setupBankExtension(queryClient);
  const stakingExtension = setupStakingExtension(queryClient);
  async function getAllBalances(address) {
    const res = await bankExtension.bank.allBalances(address);
    l();
    li(res);
    l();
    return res;
  }
  async function getMetadata(denom) {
    const res = await bankExtension.bank.denomMetadata(denom);
    l();
    li(res);
    l();
    return res;
  }
  async function getValidators() {
    const {
      validators
    } = await stakingExtension.staking.validators("BOND_STATUS_BONDED");
    l();
    li(validators);
    l();
    return validators;
  }
  return {
    getAllBalances,
    getMetadata,
    getValidators
  };
}
export { getSgExecHelpers, getSgQueryHelpers };