import { RawPriceItem } from "../codegen/Oracle.types";
import { BiddedCollateralItem, CollectionInfoForString, CurrencyForTokenUnverified, LiquidationItem } from "../codegen/LendingPlatform.types";
import { DirectSecp256k1HdWallet, OfflineSigner, OfflineDirectSigner } from "@cosmjs/proto-signing";
import { ClockSource } from "../codegen/AdapterSchedulerKujira.types";
import { TokenUnverified, QueryAllOperatorsResponse, ApprovalsResponse, TokensResponse, OwnerOfResponse } from "../interfaces";
declare function getCwExecHelpers(chainId: string, rpc: string, owner: string, signer: (OfflineSigner & OfflineDirectSigner) | DirectSecp256k1HdWallet): Promise<{
    utils: {
        cwRevoke: (collectionAddress: string, senderAddress: string, operator: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
    };
    adapter: {
        marketplace: {
            kujira: {
                cwAcceptAdminRole: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
                cwUpdateConfig: ({ admin, worker, lendingPlatform, }: {
                    admin?: string | undefined;
                    worker?: string | undefined;
                    lendingPlatform?: string | undefined;
                }, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
                cwAcceptBids: (biddedCollateralItemList: BiddedCollateralItem[], gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
            };
        };
        scheduler: {
            kujira: {
                cwAcceptAdminRole: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
                cwUpdateConfig: ({ clockSource, lendingPlatform, minClockPeriod, offchainClock, }: {
                    clockSource?: any;
                    lendingPlatform?: string | undefined;
                    minClockPeriod?: number | undefined;
                    offchainClock?: string[] | undefined;
                }, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
                cwPush: (targets: LiquidationItem[], gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
            };
        };
    };
    lending: {
        cwDeposit: (amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUnbond: (amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwWithdraw: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwWithdrawCollateral: (collections: CollectionInfoForString[], gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwApproveAndDepositCollateral: (senderAddress: string, operator: string, collections: CollectionInfoForString[], gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwBorrow: (amount: number, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwRepay: (amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwPlaceBid: (collections: CollectionInfoForString[], discount: number, amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwRemoveBid: (collectionAddresses: string[], creationDate: number, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateBid: (collections: CollectionInfoForString[], creationDate: number, amount: number, discount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwLiquidate: (targets: LiquidationItem[], gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwAcceptAdminRole: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateAddressConfig: ({ marketplace, minter, oracle, worker, scheduler, }: {
            marketplace?: string | undefined;
            minter?: string | undefined;
            oracle?: string | undefined;
            worker?: string | undefined;
            scheduler?: string | undefined;
        }, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateRateConfig: ({ bidMinRate, borrowApr, borrowFeeRate, discountMaxRate, discountMinRate, liquidationFeeRate, }: {
            bidMinRate?: number | undefined;
            borrowApr?: number | undefined;
            borrowFeeRate?: number | undefined;
            discountMaxRate?: number | undefined;
            discountMinRate?: number | undefined;
            liquidationFeeRate?: number | undefined;
        }, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateCommonConfig: ({ bglCurrency, collateralMinValue, isMarketplaceInterfaceEnabled, mainCurrency, borrowersReserveFractionRatio, priceUpdatePeriod, unbondingPeriod, }: {
            bglCurrency?: any;
            collateralMinValue?: number | undefined;
            isMarketplaceInterfaceEnabled?: boolean | undefined;
            mainCurrency?: any;
            borrowersReserveFractionRatio?: number | undefined;
            priceUpdatePeriod?: number | undefined;
            unbondingPeriod?: number | undefined;
        }, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwDepositReserveLiquidity: (amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwReinforceBglToken: (amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwLock: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUnlock: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwDistributeFunds: (addressAndWeightList: [string, number][], gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwRemoveCollection: (address: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwCreateProposal: (proposal: ProposalForStringAndTokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwRejectProposal: (id: number, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwAcceptProposal: (id: number, amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
    };
    minter: {
        cwCreateNative: (tokenOwner: string, subdenom: string, decimals: number, paymentAmount: number, paymentDenom: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwCreateCw20: (tokenOwner: string, name: string, symbol: string, marketing: InstantiateMarketingInfo, decimals: number | undefined, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwMint: (amount: number, recipient: string, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwBurn: (amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwSetMetadataNative: (token: TokenUnverified, metadata: Metadata, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateMarketingCw20: (token: TokenUnverified, description: string, marketing: string, project: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUploadLogoCw20: (token: TokenUnverified, logo: Logo, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwChangeAdminNative: (token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateMinterCw20: (token: TokenUnverified, newMinter: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwRegisterCurrency: (currency: CurrencyForTokenUnverified, creator: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwAcceptAdminRole: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateConfig: ({ cw20BaseCodeId, worker, }: {
            cw20BaseCodeId?: number | undefined;
            worker?: string | undefined;
        }, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
    };
    minterKujira: {
        cwCreateNative: (tokenOwner: string, subdenom: string, decimals: number, paymentAmount: number, paymentDenom: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwMint: (amount: number, recipient: string, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwBurn: (amount: number, token: TokenUnverified, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwRegisterCurrency: (currency: CurrencyForTokenUnverified, creator: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwAcceptAdminRole: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateConfig: ({ worker, }: {
            worker?: string | undefined;
        }, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateTokenOwner: (denom: string, owner: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
    };
    oracle: {
        cwAcceptAdminRole: (gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdateConfig: (scheduler: string, gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
        cwUpdatePrices: (data: RawPriceItem[], gasPrice: string) => Promise<import("@cosmjs/cosmwasm-stargate").DeliverTxResponse>;
    };
}>;
declare function getCwQueryHelpers(chainId: string, rpc: string): Promise<{
    utils: {
        cwQueryOperators: (ownerAddress: string, collectionAddress: string) => Promise<QueryAllOperatorsResponse>;
        cwQueryApprovals: (collectionAddress: string, tokenId: string) => Promise<ApprovalsResponse>;
        cwQueryBalanceInNft: (owner: string, collectionAddress: string) => Promise<TokensResponse>;
        cwQueryNftOwner: (collectionAddress: string, tokenId: string) => Promise<OwnerOfResponse>;
    };
    adapter: {
        marketplace: {
            kujira: {
                cwQueryConfig: () => Promise<any>;
                cwQueryLiquidationBidsByCollectionAddressList: (limit?: number, startAfter?: string) => Promise<any>;
                cwQueryLiquidationBidsByCollectionAddress: (address: string) => Promise<any>;
                cwQueryLiquidationBidsByLiquidatorAddressList: (limit?: number, startAfter?: string) => Promise<any>;
                cwQueryLiquidationBidsByLiquidatorAddress: (address: string) => Promise<any>;
            };
        };
        scheduler: {
            kujira: {
                cwQueryConfig: () => Promise<any>;
                cwQueryLog: () => Promise<any>;
            };
        };
    };
    lending: {
        cwQueryAddressConfig: () => Promise<any>;
        cwQueryRateConfig: () => Promise<any>;
        cwQueryCommonConfig: () => Promise<any>;
        cwQueryPlatformRevenue: () => Promise<any>;
        cwQueryBalances: () => Promise<any>;
        cwQueryUnbonderList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryUnbonder: (address: string) => Promise<any>;
        cwQueryBorrowerList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryBorrower: (address: string) => Promise<any>;
        cwQueryLiquidatorList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryLiquidator: (address: string) => Promise<any>;
        cwQueryCollateralList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryCollateral: (collectionAddress: string) => Promise<any>;
        cwQueryCollateralByOwner: (owner: string) => Promise<any>;
        cwQueryLiquidationBidsByCollectionAddressList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryLiquidationBidsByCollectionAddress: (address: string) => Promise<any>;
        cwQueryLiquidationBidsByLiquidatorAddressList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryLiquidationBidsByLiquidatorAddress: (address: string) => Promise<any>;
        cwQueryProposals: (limit?: number, startAfter?: number) => Promise<any>;
        cwQueryCollectionList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryCollection: (address: string) => Promise<any>;
        cwQueryBglCurrencyToMainCurrencyPrice: () => Promise<any>;
        cwQueryConditionalDepositApr: (amountToDeposit: number) => Promise<any>;
        cwQueryLtvList: (limit?: number, startAfter?: string) => Promise<any>;
        cwQueryConditionalLtv: (borrower: string, amountToDeposit?: number, amountToBorrow?: number) => Promise<any>;
        cwQueryTotalAvailableToBorrowLiquidity: () => Promise<any>;
        cwQueryAvailableToBorrow: (borrower: string, targetLtv?: number) => Promise<any>;
        cwQueryAmounts: () => Promise<any>;
    };
    minter: {
        cwQueryCurrenciesByCreator: (creator: string) => Promise<any>;
        cwQueryConfig: () => Promise<any>;
    };
    minterKujira: {
        cwQueryCurrenciesByCreator: (creator: string) => Promise<any>;
        cwQueryConfig: () => Promise<any>;
        cwQueryTokenOwner: (denom: string) => Promise<any>;
    };
    oracle: {
        cwQueryConfig: () => Promise<import("../codegen/Oracle.types").Config>;
        cwQueryPrices: (limit?: number, startAfter?: string, collectionAddresses?: string[]) => Promise<import("../codegen/Oracle.types").ArrayOfPriceItem>;
        cwQueryBlockTime: () => Promise<number>;
    };
}>;
export { getCwExecHelpers, getCwQueryHelpers };
