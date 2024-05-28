use cosmwasm_std::StdError;
use thiserror::Error;

impl From<StdError> for ContractError {
    fn from(std_error: StdError) -> Self {
        Self::CustomError {
            val: std_error.to_string(),
        }
    }
}

impl From<ContractError> for StdError {
    fn from(contract_error: ContractError) -> Self {
        Self::generic_err(contract_error.to_string())
    }
}

pub fn parse_err(err: anyhow::Error) -> StdError {
    let context = format!("{}", err);
    let source = err.source().unwrap().to_string();

    StdError::GenericErr {
        msg: format!("{}\n{}", context, source),
    }
}

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    // common
    #[error("Sender does not have access permissions!")]
    Unauthorized,

    #[error("Parameters are not provided!")]
    NoParameters,

    #[error("It's too late to accept admin role!")]
    TransferAdminDeadline,

    #[error("Not enough liquidity to claim rewards!")]
    NotEnoughLiquidity,

    #[error("Currency can not be changed after adding liquidity!")]
    ChangingCurrency,

    #[error("Platform already is added!")]
    PlatformDuplication,

    #[error("Platform is not found!")]
    PlatformIsNotFound,

    #[error("Platform is not in list!")]
    PlatformIsNotInList,

    #[error("NFT is not found!")]
    NftIsNotFound,

    #[error("NFT already is added!")]
    NftDuplication,

    #[error("Improper NFT price!")]
    ImproperNftPrice,

    #[error("Zero amount to send!")]
    ZeroAmount,

    #[error("The user doesn't have boxes!")]
    ZeroBoxAmount,

    #[error("The user doesn't have rewards!")]
    ZeroRewardsAmount,

    #[error("A user can't open multiple boxes in single tx!")]
    MultipleBoxesPerTx,

    #[error("Empty collection list!")]
    EmptyCollectionList,

    #[error("Collection already exists!")]
    CollectionDuplication,

    #[error("Collection is not found!")]
    CollectionIsNotFound,

    #[error("Sum of weights is not equal one!")]
    WeightsAreUnbalanced,

    #[error("Weight is out of range!")]
    WeightIsOutOfRange,

    #[error("Undefined Reply ID!")]
    UndefinedReplyId,

    #[error("Asset is not found!")]
    AssetIsNotFound,

    #[error("Wrong asset type!")]
    WrongAssetType,

    #[error("Improper asset amount!")]
    ImproperAssetAmount,

    #[error("Wrong message type!")]
    WrongMessageType,

    #[error("Wrong action type!")]
    WrongActionType,

    #[error("Wrong funds combination!")]
    WrongFundsCombination,

    #[error("{value:?} config is not found!")]
    ParameterIsNotFound { value: String },

    #[error("The contract is temporary locked to stop bad guys")]
    ContractIsLocked,

    #[error("Parsing previous version error!")]
    ParsingPrevVersion,

    #[error("Parsing new version error!")]
    ParsingNewVersion,

    #[error("Msg version is not equal contract new version!")]
    ImproperMsgVersion,
}
