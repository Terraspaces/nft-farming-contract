// Welcome to the Mass Key Deletion recipe.

const nearAPI = require("near-api-js"); // imports near api js
const { parseNearAmount, formatNearAmount } = require("near-api-js/lib/utils/format");

// Standard setup to connect to NEAR While using Node
const { keyStores, KeyPair, connect } = nearAPI;
const homedir = require("os").homedir();
const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = require("path").join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

let config;

const configSetting = "mainnet";

const GAS_FOR_NFT_APPROVE = "20000000000000";
const GAS_FOR_RESOLVE_TRANSFER = "10000000000000";
const GAS_FOR_NFT_TRANSFER_CALL = "300000000000000";
const MAX_GAS = "300000000000000";
const DEPOSIT = "540000000000000000000";
// setting configuration based on input

const token_rate = [];
const Rate_Data = {
  Kryptonite: 14,
  Lunar: 8,
  Quartz: 6,
  Iceberg: 4,
  Golden: 3,
  Terra: 2
};

const fetchTrait = async () => {
  for (let i = 1; i <= 777; i++) {
    const result = await fetch("https://terraspaces_nft_1.mypinata.cloud/ipfs/QmeP2Gn7fjycGerqTiKZnexyYXvu5qvDVKq4WHdfzwL8bi/" + i + ".json");
    const metadata = (await result.json())["attributes"];
    for (let j = 0; j < metadata.length; j++) {
      if (metadata[j]["trait_type"] == 'Terrain') {
        token_rate.push({ token_id: i.toString(), rate: Rate_Data[metadata[j]["value"]] });
        console.log(i, { token_id: i.toString(), rate: Rate_Data[metadata[j]["value"]] });
        break;
      }
    }
  }
}


switch (configSetting) {
  case "mainnet":
    config = {
      networkId: "mainnet",
      keyStore, // optional if not signing transactions
      nodeUrl: "https://rpc.mainnet.near.org",
      walletUrl: "https://wallet.near.org",
      helperUrl: "https://helper.mainnet.near.org",
      explorerUrl: "https://explorer.mainnet.near.org",
    };
    console.log("configuration set to mainnet ");

    break;

  case "testnet":
    config = {
      networkId: "testnet",
      keyStore, // optional if not signing transactions
      nodeUrl: "https://rpc.testnet.near.org",
      walletUrl: "https://wallet.testnet.near.org",
      helperUrl: "https://helper.testnet.near.org",
      explorerUrl: "https://explorer.testnet.near.org",
    };
    console.log("configuration set to testnet ");
    break;
  default:
    console.log(`please choose a configuration `);
}

const FARM_CONTRACT_ID = "terraspaces-farming.near";
const NFT_CONTRACT_ID = "terraspaces.near";

const doTransaction = async () => {
  //Load Your Account
  const near = await connect(config);
  const xu_account = await near.account("xuguangxia.near");
  const farm_account = await near.account(FARM_CONTRACT_ID);

  // await fetchTrait();

  // for (let i = 0; i < token_rate.length; i++) {
  //   await xu_account.functionCall({
  //     contractId: FARM_CONTRACT_ID,
  //     methodName: "set_token_rate",
  //     args: {
  //       nft_contract_id: NFT_CONTRACT_ID,
  //       token_id: token_rate[i].token_id,
  //       rate: token_rate[i].rate.toString()
  //     },
  //     gas: GAS_FOR_RESOLVE_TRANSFER,
  //     attachedDeposit: "0"
  //   });
  //   console.log("TokenRate Set", i);
  // }

  // let balance = await farm_account.viewFunction("usn", "ft_balance_of", {
  //   account_id: farm_account.accountId
  // });
  // console.log("Balance:", balance);

  // await farm_account.functionCall({
  //   contractId: "usn",
  //   methodName: "ft_transfer",
  //   args: {
  //     receiver_id: xu_account.accountId,
  //     amount: parseNearAmount("0.00349"),
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: "1"
  // });
  // console.log("USN returned");

  // await xu_account.functionCall({
  //   contractId: NFT_CONTRACT_ID,
  //   methodName: "nft_approve",
  //   args: {
  //     token_id: "10",
  //     account_id: FARM_CONTRACT_ID,
  //     msg: JSON.stringify({ staking_status: "Staking to Farm" })
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: DEPOSIT
  // });
  // console.log("Staked");

  // await xu_account.functionCall({
  //   contractId: FARM_CONTRACT_ID,
  //   methodName: "unstake",
  //   args: {
  //     nft_contract_id: NFT_CONTRACT_ID,
  //     token_id: "10"
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: "1"
  // });
  // console.log("Unstaked");

  // await xu_account.functionCall({
  //   contractId: FARM_CONTRACT_ID,
  //   methodName: "claim_reward",
  //   args: {
  //     nft_contract_id: NFT_CONTRACT_ID,
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: "1"
  // });
  // console.log("Claimed");

  let rate = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_token_rate", {
    nft_contract_id: NFT_CONTRACT_ID,
    token_id: "44"
  });
  console.log("Rate:", rate);

  // let supply = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_supply_by_owner_id", {
  //   account_id: xu_account.accountId,
  //   nft_contract_id: NFT_CONTRACT_ID
  // });
  // console.log("Supply:", supply);

  // let farm_supply = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_supply_farm", {
  //   nft_contract_id: NFT_CONTRACT_ID
  // });
  // console.log("FarmSupply:", farm_supply);

  // let stake_info = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_staking_informations_by_owner_id", {
  //   account_id: xu_account.accountId,
  //   nft_contract_id: NFT_CONTRACT_ID
  // });
  // console.log("StakeInfo:", stake_info);

  // let claim_amount = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_claim_amount_by_owner_id", {
  //   account_id: xu_account.accountId,
  //   nft_contract_id: NFT_CONTRACT_ID
  // });
  // console.log("ClaimAmount:", formatNearAmount(claim_amount + "000000"));

};

doTransaction();

