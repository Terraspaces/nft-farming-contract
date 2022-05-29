// Welcome to the Mass Key Deletion recipe.

const nearAPI = require("near-api-js"); // imports near api js
const { parseNearAmount } = require("near-api-js/lib/utils/format");

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
const DEPOSIT = "450000000000000000000";

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

const MARKETPLACE_CONTRACT_ID = "alphaart_test_6.xuguangxia.testnet";
const NFT_CONTRACT_ID = "launchpad_test_4.xuguangxia.testnet";

const viewFunction = async () => {
  //Load Your Account
  const near = await connect(config);
  const xu_account = await near.account("xuguangxia.testnet");
  const temp_account = await near.account("pixel8testing2.testnet");

  // await xu_account.functionCall({
  //   contractId: NFT_CONTRACT_ID,
  //   methodName: "nft_transfer_call",
  //   args: {
  //     receiver_id: MARKETPLACE_CONTRACT_ID,
  //     token_id: "2",
  //     msg: JSON.stringify({ sale_conditions: { "near": parseNearAmount("1.0") } })
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: "1"
  // });
  // console.log("Listed");

  // await temp_account.functionCall({
  //   contractId: MARKETPLACE_CONTRACT_ID,
  //   methodName: "buy",
  //   args: {
  //     nft_contract_id: NFT_CONTRACT_ID,
  //     token_id: "2"
  //   },
  //   gas: GAS_FOR_NFT_TRANSFER_CALL,
  //   attachedDeposit: parseNearAmount("1.0")
  // });
  // console.log("Purchased");

  await temp_account.functionCall({
    contractId: MARKETPLACE_CONTRACT_ID,
    methodName: "offer",
    args: {
      nft_contract_id: NFT_CONTRACT_ID,
      token_id: "7"
    },
    gas: GAS_FOR_NFT_TRANSFER_CALL,
    attachedDeposit: parseNearAmount("1.0")
  });
  console.log("Offered");

  let offers = await temp_account.viewFunction(MARKETPLACE_CONTRACT_ID, "get_offered_tokens_by_owner_id", {
    account_id: temp_account.accountId,
    from_index: "0",
    limit: 10
  });
  console.log("Offers:", offers);

  for (let i = 0; i < offers.length; i++) {
    let offer = await temp_account.viewFunction(MARKETPLACE_CONTRACT_ID, "get_offers", {
      nft_contract_token: NFT_CONTRACT_ID + "||" + "7",
    });
    console.log("Offer:", offer);
  }

  // await xu_account.functionCall({
  //   contractId: NFT_CONTRACT_ID,
  //   methodName: "nft_approve",
  //   args: {
  //     token_id: "6",
  //     account_id: MARKETPLACE_CONTRACT_ID,
  //     msg: JSON.stringify({ approve_type: "AcceptOffer", account_id: temp_account.accountId, price: parseNearAmount("1.0") })
  //   },
  //   gas: MAX_GAS,
  //   attachedDeposit: parseNearAmount("0.001")
  // });
  // console.log("Accept Offer");

};

viewFunction();
