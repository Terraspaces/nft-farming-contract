near create-account terraspaces_farm_test_4.xuguangxia.testnet --masterAccount xuguangxia.testnet --initialBalance 5

near deploy --accountId terraspaces_farm_test_4.xuguangxia.testnet --wasmFile out/nft_farming.wasm --initFunction new --initArgs '{"owner_id": "xuguangxia.testnet"}' --initGas '300000000000000'
