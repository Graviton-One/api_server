pub struct Constants {
    pub zero_address: &'static str,
    pub balance_keeper: &'static str,
    pub voter: &'static str,
    pub lp_keeper: &'static str,
    pub oracle_router: &'static str,
    pub oracle_parser: &'static str,
    pub balance_adder: &'static str,
    pub farm_eb: &'static str,
    pub shares_eb: &'static str,
    pub farm_staking: &'static str,
    pub claim_gton: &'static str,
    pub topic0_set_owner: &'static str,
    pub topic0_set_can_open: &'static str,
    pub topic0_set_can_add: &'static str,
    pub topic0_set_can_subtract: &'static str,
    pub topic0_balance_keeper_open: &'static str,
    pub topic0_balance_keeper_add: &'static str,
    pub topic0_balance_keeper_subtract: &'static str,
    pub topic0_voter_set_can_cast_votes: &'static str,
    pub topic0_voter_set_can_check: &'static str,
    pub topic0_voter_start_round: &'static str,
    pub topic0_voter_finalize_round: &'static str,
    pub topic0_voter_cast_votes: &'static str,
    pub topic0_voter_check_vote_balance: &'static str,
    pub topic0_lp_keeper_add: &'static str,
    pub topic0_lp_keeper_subtract: &'static str,
    pub topic0_set_can_route: &'static str,
    pub topic0_route_value: &'static str,
    pub topic0_set_nebula: &'static str,
    pub topic0_set_router: &'static str,
    pub topic0_set_is_evm: &'static str,
    pub topic0_attach_value: &'static str,
    pub topic0_add_farm: &'static str,
    pub topic0_remove_farm: &'static str,
    pub topic0_process_balances: &'static str,
    pub topic0_process_balance: &'static str,
    pub topic0_set_voter: &'static str,
    pub topic0_set_wallet: &'static str,
    pub topic0_claim: &'static str,
    pub topic0_lock_gton: &'static str,
    pub topic0_set_can_lock: &'static str,
    pub topic0_migrate: &'static str,
    pub topic0_lp_lock: &'static str,
    pub topic0_lp_unlock: &'static str,
    pub topic0_set_is_allowed_token: &'static str,
    pub topic0_set_lock_limit: &'static str,
    pub balance_keeper_deploy: u64,
    pub lp_keeper_deploy: u64,
}

pub static C: Constants = Constants {
    zero_address: "0x0000000000000000000000000000000000000000",
    balance_keeper: "0x4AB096F49F2Af3cfcf2D851094FA5936f18aed90",
    voter: "0xbf7F3Db33CfaEFA2c8d44cADc5D38d5899B2a1FF",
    lp_keeper: "0xA0447eE66E44BF567FF9287107B0c3D2F88efD93",
    oracle_router: "0x1b3223c54f04543Bc656a2C2C127576F314b5449",
    oracle_parser: "0x7fCCE1303F7e1fc14780C87F6D67346EC44a4027",
    balance_adder: "0x8d712f350A55D65427EfcE56Ec6a36fef28e8Ac9",
    farm_eb: "0x4Cb8824d45312D5dC9d9B5260fb2B1dEC297015b",
    shares_eb: "0x521C9352E2782c947F4354179D144f09D8c0b0c3",
    farm_staking: "0x99587ecA8b1A371e673601E2a4a1be7a65F74867",
    claim_gton: "0xC1BD11Fd4309b5E40BBcf5fD350E2DceF707f66c",
    topic0_set_owner: "0xcbf985117192c8f614a58aaf97226bb80a754772f5f6edf06f87c675f2e6c663",
    topic0_set_can_open: "0x1317484b2184978eae33164465cb4df6d4e79982c6516a36ec4ada226eb1d345",
    topic0_set_can_add: "0xcfc65705d7d3022a56eae794c52cdedeac637251211c8c3345af78eee9cbe546",
    topic0_set_can_subtract: "0xc954f3ae852ebf9ca68e929d8e491727fcd9893a8c84c053fcf2a2637b8d5000",
    topic0_balance_keeper_open: "0x50042401acb675fedd6dd939fccc629832ce60fa4185df25c01b20e8def195bf",
    topic0_balance_keeper_add: "0xc264f49177bdbe55a01fae0e77c3fdc75d515d242b32bc4d56c565f5b47865ba",
    topic0_balance_keeper_subtract: "0x47dd4a08dedb9e7afcf164b736d2a3fcaed9f9d56ec8d5e38aaafde8c22a58a7",
    topic0_voter_set_can_cast_votes: "0xfb3bc4219a56f26314da817cfc2d275075df8efc289db884e65b8d08dc8b1551",
    topic0_voter_set_can_check: "0xbe36b0226217340762183bac9b95c087d57582fa89fe1f558bdcae4f01ca5c52",
    topic0_voter_start_round: "0x5e1a5715172c93eabac8e43d1d6d25a16666a54bdbe86ad319d89bcaa52f11fb",
    topic0_voter_finalize_round: "0x2237c4c7f7b065c0962407d5e7a8e049fb9f76823365bc367a6c276756490ed6",
    topic0_voter_cast_votes: "0xb0203c31371583d54e7dd44f55f6a78e3ef2964a045dbe15ea4b47ba2a4814af",
    topic0_voter_check_vote_balance: "0xc6a27c61104a378462b1fd982971d5cc85a9d2300e9aa0636396643b54c64762",
    topic0_lp_keeper_add: "0xa2ba758028b7d21eee988892461f8630fe375441d69998d867174616c5eef8fb",
    topic0_lp_keeper_subtract: "0xb271e8aacff5df8ac4dd0fc1a4da3cc1d39edb125050765b9ed0e5103af9fa3c",
    topic0_set_can_route: "0x20ca82784c484f1f8c3a8e759a0907d411b81a5d563db4d1b2d698165201772c",
    topic0_route_value: "0x112b98d9b7f0fd96e462b22deffd3ec2c95405e8458b249860f64e8a6ebf4b59",
    topic0_set_nebula: "0x93cc977e7dfe8ecc1f529e8fc1ff4b39e350da4788a61b6552ddf669160c8f73",
    topic0_set_router: "0xdd818201669f7032dfbf00808df052dbc6ddb9255dc68a5a3233b1c8d73a724b",
    topic0_set_is_evm: "0x91687ab4d04768930bd62b7b99d3d92d04cc73fe0012d3654c883c9bed496d86",
    topic0_attach_value: "0xdedc6edbc70712376c85a9aafdc87bbce69a70b6f789a345558447c88785b553",
    topic0_add_farm: "0xb84ff72f17ef878c5e270c59d160083239fa9fbff6111441cc84cdc07fda4b05",
    topic0_remove_farm: "0x084f61398b65df2c019793a91ca836dafcd56ac22adf2cc501c98e44665457da",
    topic0_process_balances: "0x58daf0f3614604151fd17c5915e68241308b3404ac4c96a3af636acd06b3e4df",
    topic0_process_balance: "0x67da84f1d8d6e06e895499674121e08e062f1f9ff352d9913338ad9b6c30a47a",
    topic0_set_voter: "0x0b62bd6c4eddf46a28330fdf3d6b99ce90b7d760ba59bacda81f3b74d889378c",
    topic0_set_wallet: "0x8595877311e370fe3ac87d4f6d12473603393f02ac660e68d2e5e3da5adb610c",
    topic0_claim: "0x70eb43c4a8ae8c40502dcf22436c509c28d6ff421cf07c491be56984bd987068",
    topic0_lock_gton: "0x321236f8c59ef7b2aa21ee1f2a787e0405dd3a829359f51f2ddb3aeabeb3004a",
    topic0_set_can_lock: "0x3a3aa2ba0b9c1822d134b9a463f7f8c87f7dc45650e4860761ab9367e3c47026",
    topic0_migrate: "0xa59785389b00cbd19745afbe8d59b28e3161395c6b1e3525861a2b0dede0b90d",
    topic0_lp_lock: "0xd6aba49fa5adb7dbc18ab12d057e77c75e5d4b345cf473c7514afbbd6f5fc626",
    topic0_lp_unlock: "0xd99169b5dcb595fb976fee14578e44584c0ebbbf50cf58d568b3100c59f2f4bb",
    topic0_set_is_allowed_token: "0x28e2bc4df2fbb2719002ab1b99e1fcffe8be886652c8f563fcbe8ff38d2de5d3",
    topic0_set_lock_limit: "0xd0ea9b960a754ff500179f96481ffd92bc90e886f2d893d17c7affdb180db646",
    balance_keeper_deploy: 11332355,
    lp_keeper_deploy: 11332621,
};

