near create-account terraspaces_farm_test_4.xuguangxia.near --masterAccount xuguangxia.near --initialBalance 5

near deploy --accountId terraspaces-farming.near --wasmFile out/nft_farming.wasm --initFunction new --initArgs '{"owner_id": "xuguangxia.near"}' --initGas '300000000000000'
