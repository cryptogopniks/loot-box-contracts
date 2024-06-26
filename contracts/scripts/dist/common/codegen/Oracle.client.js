/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export class OracleQueryClient {
  constructor(client, contractAddress) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.queryConfig = this.queryConfig.bind(this);
    this.queryPrices = this.queryPrices.bind(this);
    this.queryBlockTime = this.queryBlockTime.bind(this);
  }
  queryConfig = async () => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_config: {}
    });
  };
  queryPrices = async ({
    collectionAddresses,
    limit,
    startAfter
  }) => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_prices: {
        collection_addresses: collectionAddresses,
        limit,
        start_after: startAfter
      }
    });
  };
  queryBlockTime = async () => {
    return this.client.queryContractSmart(this.contractAddress, {
      query_block_time: {}
    });
  };
}
export class OracleClient extends OracleQueryClient {
  constructor(client, sender, contractAddress) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.acceptAdminRole = this.acceptAdminRole.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.updatePrices = this.updatePrices.bind(this);
  }
  acceptAdminRole = async (fee = "auto", memo, _funds) => {
    return await this.client.execute(this.sender, this.contractAddress, {
      accept_admin_role: {}
    }, fee, memo, _funds);
  };
  updateConfig = async ({
    admin,
    scheduler,
    worker
  }, fee = "auto", memo, _funds) => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_config: {
        admin,
        scheduler,
        worker
      }
    }, fee, memo, _funds);
  };
  updatePrices = async ({
    data
  }, fee = "auto", memo, _funds) => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_prices: {
        data
      }
    }, fee, memo, _funds);
  };
}