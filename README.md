Near Program for PiggyDao's Alpha Art

This need to working on Ubuntu OS.
https://www.near-sdk.io/
here you can check how to install Rust Toolchain.
https://github.com/near/near-cli
here you can check how to install Near CLI.

1. compile smart contract to wasm file.

cd contracts/market && ./build.sh && cd ../..

You can check market.wasm file appeared in ./out/

Now we can move on to the deployment phase.

2. deploy smart contract to Near Protocol.

Typically Near CLI is working with testnet
You can switch it with below command.

export NEAR_ENV=mainnet

And also you need to login with Near CLI to your wallet.
https://docs.near.org/docs/tools/near-cli#access-keys

You can deploy the contract to a subaccount which made from your main account.
If you want to make a sub account, You can use this method

near create-account alphaart_test_6.xuguangxia.testnet --masterAccount xuguangxia.testnet --initialBalance 5


You can deploy contract with below command.
near deploy --accountId alphaart_test_6.xuguangxia.testnet --wasmFile out/escrow.wasm --initFunction new --initArgs '{"owner_id": "xuguangxia.testnet"}' --initGas '300000000000000'

3. I am leaving required links for the Near Integration.
First of all you need to install near-api-js module.

npm install near-api-js

You can through basic concepts here.
https://docs.near.org/docs/develop/front-end/near-api-js

Plz check these urls
https://github.com/TwoSteppeDigital-Org/Galacticway_io/blob/main/src/contexts/connection.tsx(wallet provider)
https://github.com/TwoSteppeDigital-Org/Galacticway_io/blob/main/src/pages/admin/AddCollectionPage/AddCollectionPage.tsx(contract integration)
https://github.com/TwoSteppeDigital-Org/Galacticway_io/blob/main/src/components/AppNavbar/AppNavbar.tsx(wallet connection)
