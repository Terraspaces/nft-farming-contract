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

const configSetting = "testnet";

const GAS_FOR_NFT_APPROVE = "20000000000000";
const GAS_FOR_RESOLVE_TRANSFER = "10000000000000";
const GAS_FOR_NFT_TRANSFER_CALL = "300000000000000";
const MAX_GAS = "300000000000000";
const DEPOSIT = "540000000000000000000";
// setting configuration based on input
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

const FARM_CONTRACT_ID = "terraspaces_farm_test_1.xuguangxia.testnet";
const NFT_CONTRACT_ID = "launchpad_test_4.xuguangxia.testnet";

const viewFunction = async () => {
  //Load Your Account
  const near = await connect(config);
  const xu_account = await near.account("xuguangxia.testnet");
  const farm_account = await near.account(FARM_CONTRACT_ID);

  // let balance = await farm_account.viewFunction("usdn.testnet", "ft_balance_of", {
  //   account_id: farm_account.accountId
  // });
  // console.log("Balance:", balance);

  // await farm_account.functionCall({
  //   contractId: "usdn.testnet",
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
  //     token_id: "4",
  //     account_id: FARM_CONTRACT_ID,
  //     msg: JSON.stringify({ staking_status: "Staking to Farm" })
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: DEPOSIT
  // });
  // console.log("Staked");

  // await xu_account.functionCall({
  //   contractId: FARM_CONTRACT_ID,
  //   methodName: "claim_reward",
  //   args: {
  //     nft_contract_id: NFT_CONTRACT_ID,
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: "1"
  // });
  // console.log("Staked");

  // await xu_account.functionCall({
  //   contractId: FARM_CONTRACT_ID,
  //   methodName: "unstake",
  //   args: {
  //     nft_contract_id: NFT_CONTRACT_ID,
  //     token_id: "3"
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: "1"
  // });
  // console.log("Staked");

  let supply = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_supply_by_owner_id", {
    account_id: xu_account.accountId,
    nft_contract_id: NFT_CONTRACT_ID
  });
  console.log("Supply:", supply);

  let farm_supply = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_supply_farm", {
    nft_contract_id: NFT_CONTRACT_ID
  });
  console.log("FarmSupply:", farm_supply);

  let stake_info = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_staking_informations_by_owner_id", {
    account_id: xu_account.accountId,
    nft_contract_id: NFT_CONTRACT_ID
  });
  console.log("StakeInfo:", stake_info);

  let claim_amount = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_claim_amount_by_owner_id", {
    account_id: xu_account.accountId,
    nft_contract_id: NFT_CONTRACT_ID
  });
  console.log("ClaimAmount:", formatNearAmount(claim_amount));

  // for (let i = 0; i < offers.length; i++) {
  //   let offer = await xu_account.viewFunction(FARM_CONTRACT_ID, "get_offers", {
  //     nft_contract_token: NFT_CONTRACT_ID + "||" + "7",
  //   });
  //   console.log("Offer:", offer);
  // }

  // await xu_account.functionCall({
  //   contractId: NFT_CONTRACT_ID,
  //   methodName: "nft_approve",
  //   args: {
  //     token_id: "6",
  //     account_id: FARM_CONTRACT_ID,
  //     msg: JSON.stringify({ approve_type: "AcceptOffer", account_id: xu_account.accountId, price: parseNearAmount("1.0") })
  //   },
  //   gas: MAX_GAS,
  //   attachedDeposit: parseNearAmount("0.001")
  // });
  // console.log("Accept Offer");

};

viewFunction();
