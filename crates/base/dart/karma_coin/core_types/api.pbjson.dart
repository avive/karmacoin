///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use submitTransactionResultDescriptor instead')
const SubmitTransactionResult$json = const {
  '1': 'SubmitTransactionResult',
  '2': const [
    const {'1': 'SUBMIT_TRANSACTION_RESULT_REJECTED', '2': 0},
    const {'1': 'SUBMIT_TRANSACTION_RESULT_SUBMITTED', '2': 1},
  ],
};

/// Descriptor for `SubmitTransactionResult`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List submitTransactionResultDescriptor = $convert.base64Decode('ChdTdWJtaXRUcmFuc2FjdGlvblJlc3VsdBImCiJTVUJNSVRfVFJBTlNBQ1RJT05fUkVTVUxUX1JFSkVDVEVEEAASJwojU1VCTUlUX1RSQU5TQUNUSU9OX1JFU1VMVF9TVUJNSVRURUQQAQ==');
@$core.Deprecated('Use getExchangeRateRequestDescriptor instead')
const GetExchangeRateRequest$json = const {
  '1': 'GetExchangeRateRequest',
};

/// Descriptor for `GetExchangeRateRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getExchangeRateRequestDescriptor = $convert.base64Decode('ChZHZXRFeGNoYW5nZVJhdGVSZXF1ZXN0');
@$core.Deprecated('Use getExchangeRateResponseDescriptor instead')
const GetExchangeRateResponse$json = const {
  '1': 'GetExchangeRateResponse',
  '2': const [
    const {'1': 'exchange_rate', '3': 1, '4': 1, '5': 1, '10': 'exchangeRate'},
  ],
};

/// Descriptor for `GetExchangeRateResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getExchangeRateResponseDescriptor = $convert.base64Decode('ChdHZXRFeGNoYW5nZVJhdGVSZXNwb25zZRIjCg1leGNoYW5nZV9yYXRlGAEgASgBUgxleGNoYW5nZVJhdGU=');
@$core.Deprecated('Use getUserInfoByUserNameRequestDescriptor instead')
const GetUserInfoByUserNameRequest$json = const {
  '1': 'GetUserInfoByUserNameRequest',
  '2': const [
    const {'1': 'user_name', '3': 1, '4': 1, '5': 9, '10': 'userName'},
  ],
};

/// Descriptor for `GetUserInfoByUserNameRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getUserInfoByUserNameRequestDescriptor = $convert.base64Decode('ChxHZXRVc2VySW5mb0J5VXNlck5hbWVSZXF1ZXN0EhsKCXVzZXJfbmFtZRgBIAEoCVIIdXNlck5hbWU=');
@$core.Deprecated('Use getUserInfoByUserNameResponseDescriptor instead')
const GetUserInfoByUserNameResponse$json = const {
  '1': 'GetUserInfoByUserNameResponse',
  '2': const [
    const {'1': 'user', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.User', '10': 'user'},
  ],
};

/// Descriptor for `GetUserInfoByUserNameResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getUserInfoByUserNameResponseDescriptor = $convert.base64Decode('Ch1HZXRVc2VySW5mb0J5VXNlck5hbWVSZXNwb25zZRIvCgR1c2VyGAEgASgLMhsua2FybWFfY29pbi5jb3JlX3R5cGVzLlVzZXJSBHVzZXI=');
@$core.Deprecated('Use submitTransactionRequestDescriptor instead')
const SubmitTransactionRequest$json = const {
  '1': 'SubmitTransactionRequest',
  '2': const [
    const {'1': 'transaction', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.SignedTransaction', '10': 'transaction'},
  ],
};

/// Descriptor for `SubmitTransactionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List submitTransactionRequestDescriptor = $convert.base64Decode('ChhTdWJtaXRUcmFuc2FjdGlvblJlcXVlc3QSSgoLdHJhbnNhY3Rpb24YASABKAsyKC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuU2lnbmVkVHJhbnNhY3Rpb25SC3RyYW5zYWN0aW9u');
@$core.Deprecated('Use submitTransactionResponseDescriptor instead')
const SubmitTransactionResponse$json = const {
  '1': 'SubmitTransactionResponse',
  '2': const [
    const {'1': 'submit_transaction_result', '3': 1, '4': 1, '5': 14, '6': '.karma_coin.api.SubmitTransactionResult', '10': 'submitTransactionResult'},
  ],
};

/// Descriptor for `SubmitTransactionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List submitTransactionResponseDescriptor = $convert.base64Decode('ChlTdWJtaXRUcmFuc2FjdGlvblJlc3BvbnNlEmMKGXN1Ym1pdF90cmFuc2FjdGlvbl9yZXN1bHQYASABKA4yJy5rYXJtYV9jb2luLmFwaS5TdWJtaXRUcmFuc2FjdGlvblJlc3VsdFIXc3VibWl0VHJhbnNhY3Rpb25SZXN1bHQ=');
@$core.Deprecated('Use getUserInfoByNumberRequestDescriptor instead')
const GetUserInfoByNumberRequest$json = const {
  '1': 'GetUserInfoByNumberRequest',
  '2': const [
    const {'1': 'mobile_number', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'mobileNumber'},
  ],
};

/// Descriptor for `GetUserInfoByNumberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getUserInfoByNumberRequestDescriptor = $convert.base64Decode('ChpHZXRVc2VySW5mb0J5TnVtYmVyUmVxdWVzdBJICg1tb2JpbGVfbnVtYmVyGAEgASgLMiMua2FybWFfY29pbi5jb3JlX3R5cGVzLk1vYmlsZU51bWJlclIMbW9iaWxlTnVtYmVy');
@$core.Deprecated('Use getUserInfoByNumberResponseDescriptor instead')
const GetUserInfoByNumberResponse$json = const {
  '1': 'GetUserInfoByNumberResponse',
  '2': const [
    const {'1': 'user', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.User', '10': 'user'},
  ],
};

/// Descriptor for `GetUserInfoByNumberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getUserInfoByNumberResponseDescriptor = $convert.base64Decode('ChtHZXRVc2VySW5mb0J5TnVtYmVyUmVzcG9uc2USLwoEdXNlchgBIAEoCzIbLmthcm1hX2NvaW4uY29yZV90eXBlcy5Vc2VyUgR1c2Vy');
@$core.Deprecated('Use getUserInfoByAccountRequestDescriptor instead')
const GetUserInfoByAccountRequest$json = const {
  '1': 'GetUserInfoByAccountRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
  ],
};

/// Descriptor for `GetUserInfoByAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getUserInfoByAccountRequestDescriptor = $convert.base64Decode('ChtHZXRVc2VySW5mb0J5QWNjb3VudFJlcXVlc3QSPwoKYWNjb3VudF9pZBgBIAEoCzIgLmthcm1hX2NvaW4uY29yZV90eXBlcy5BY2NvdW50SWRSCWFjY291bnRJZA==');
@$core.Deprecated('Use getUserInfoByAccountResponseDescriptor instead')
const GetUserInfoByAccountResponse$json = const {
  '1': 'GetUserInfoByAccountResponse',
  '2': const [
    const {'1': 'user', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.User', '10': 'user'},
  ],
};

/// Descriptor for `GetUserInfoByAccountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getUserInfoByAccountResponseDescriptor = $convert.base64Decode('ChxHZXRVc2VySW5mb0J5QWNjb3VudFJlc3BvbnNlEi8KBHVzZXIYASABKAsyGy5rYXJtYV9jb2luLmNvcmVfdHlwZXMuVXNlclIEdXNlcg==');
@$core.Deprecated('Use getGenesisDataRequestDescriptor instead')
const GetGenesisDataRequest$json = const {
  '1': 'GetGenesisDataRequest',
};

/// Descriptor for `GetGenesisDataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getGenesisDataRequestDescriptor = $convert.base64Decode('ChVHZXRHZW5lc2lzRGF0YVJlcXVlc3Q=');
@$core.Deprecated('Use getGenesisDataResponseDescriptor instead')
const GetGenesisDataResponse$json = const {
  '1': 'GetGenesisDataResponse',
  '2': const [
    const {'1': 'net_id', '3': 1, '4': 1, '5': 13, '10': 'netId'},
    const {'1': 'net_name', '3': 2, '4': 1, '5': 9, '10': 'netName'},
    const {'1': 'genesis_time', '3': 3, '4': 1, '5': 4, '10': 'genesisTime'},
    const {'1': 'signup_reward_phase1_alloc', '3': 4, '4': 1, '5': 4, '10': 'signupRewardPhase1Alloc'},
    const {'1': 'signup_reward_phase2_alloc', '3': 5, '4': 1, '5': 4, '10': 'signupRewardPhase2Alloc'},
    const {'1': 'signup_reward_phase1_amount', '3': 6, '4': 1, '5': 4, '10': 'signupRewardPhase1Amount'},
    const {'1': 'signup_reward_phase2_amount', '3': 7, '4': 1, '5': 4, '10': 'signupRewardPhase2Amount'},
    const {'1': 'signup_reward_phase3_start', '3': 8, '4': 1, '5': 4, '10': 'signupRewardPhase3Start'},
    const {'1': 'referral_reward_phase1_alloc', '3': 9, '4': 1, '5': 4, '10': 'referralRewardPhase1Alloc'},
    const {'1': 'referral_reward_phase2_alloc', '3': 10, '4': 1, '5': 4, '10': 'referralRewardPhase2Alloc'},
    const {'1': 'referral_reward_phase1_amount', '3': 11, '4': 1, '5': 4, '10': 'referralRewardPhase1Amount'},
    const {'1': 'referral_reward_phase2_amount', '3': 12, '4': 1, '5': 4, '10': 'referralRewardPhase2Amount'},
    const {'1': 'tx_fee_subsidy_max_per_user', '3': 13, '4': 1, '5': 4, '10': 'txFeeSubsidyMaxPerUser'},
    const {'1': 'tx_fee_subsidies_alloc', '3': 14, '4': 1, '5': 4, '10': 'txFeeSubsidiesAlloc'},
    const {'1': 'tx_fee_subsidy_max_amount', '3': 15, '4': 1, '5': 4, '10': 'txFeeSubsidyMaxAmount'},
    const {'1': 'block_reward_amount', '3': 16, '4': 1, '5': 4, '10': 'blockRewardAmount'},
    const {'1': 'block_reward_last_block', '3': 17, '4': 1, '5': 4, '10': 'blockRewardLastBlock'},
    const {'1': 'karma_reward_amount', '3': 18, '4': 1, '5': 4, '10': 'karmaRewardAmount'},
    const {'1': 'karma_reward_alloc', '3': 19, '4': 1, '5': 4, '10': 'karmaRewardAlloc'},
    const {'1': 'karma_reward_top_n_users', '3': 20, '4': 1, '5': 4, '10': 'karmaRewardTopNUsers'},
    const {'1': 'treasury_premint_amount', '3': 21, '4': 1, '5': 4, '10': 'treasuryPremintAmount'},
    const {'1': 'treasury_account_id', '3': 22, '4': 1, '5': 9, '10': 'treasuryAccountId'},
    const {'1': 'treasury_account_name', '3': 23, '4': 1, '5': 9, '10': 'treasuryAccountName'},
    const {'1': 'char_traits', '3': 24, '4': 3, '5': 11, '6': '.karma_coin.core_types.CharTrait', '10': 'charTraits'},
    const {'1': 'verifiers', '3': 25, '4': 3, '5': 11, '6': '.karma_coin.core_types.PhoneVerifier', '10': 'verifiers'},
  ],
};

/// Descriptor for `GetGenesisDataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getGenesisDataResponseDescriptor = $convert.base64Decode('ChZHZXRHZW5lc2lzRGF0YVJlc3BvbnNlEhUKBm5ldF9pZBgBIAEoDVIFbmV0SWQSGQoIbmV0X25hbWUYAiABKAlSB25ldE5hbWUSIQoMZ2VuZXNpc190aW1lGAMgASgEUgtnZW5lc2lzVGltZRI7ChpzaWdudXBfcmV3YXJkX3BoYXNlMV9hbGxvYxgEIAEoBFIXc2lnbnVwUmV3YXJkUGhhc2UxQWxsb2MSOwoac2lnbnVwX3Jld2FyZF9waGFzZTJfYWxsb2MYBSABKARSF3NpZ251cFJld2FyZFBoYXNlMkFsbG9jEj0KG3NpZ251cF9yZXdhcmRfcGhhc2UxX2Ftb3VudBgGIAEoBFIYc2lnbnVwUmV3YXJkUGhhc2UxQW1vdW50Ej0KG3NpZ251cF9yZXdhcmRfcGhhc2UyX2Ftb3VudBgHIAEoBFIYc2lnbnVwUmV3YXJkUGhhc2UyQW1vdW50EjsKGnNpZ251cF9yZXdhcmRfcGhhc2UzX3N0YXJ0GAggASgEUhdzaWdudXBSZXdhcmRQaGFzZTNTdGFydBI/ChxyZWZlcnJhbF9yZXdhcmRfcGhhc2UxX2FsbG9jGAkgASgEUhlyZWZlcnJhbFJld2FyZFBoYXNlMUFsbG9jEj8KHHJlZmVycmFsX3Jld2FyZF9waGFzZTJfYWxsb2MYCiABKARSGXJlZmVycmFsUmV3YXJkUGhhc2UyQWxsb2MSQQodcmVmZXJyYWxfcmV3YXJkX3BoYXNlMV9hbW91bnQYCyABKARSGnJlZmVycmFsUmV3YXJkUGhhc2UxQW1vdW50EkEKHXJlZmVycmFsX3Jld2FyZF9waGFzZTJfYW1vdW50GAwgASgEUhpyZWZlcnJhbFJld2FyZFBoYXNlMkFtb3VudBI7Cht0eF9mZWVfc3Vic2lkeV9tYXhfcGVyX3VzZXIYDSABKARSFnR4RmVlU3Vic2lkeU1heFBlclVzZXISMwoWdHhfZmVlX3N1YnNpZGllc19hbGxvYxgOIAEoBFITdHhGZWVTdWJzaWRpZXNBbGxvYxI4Chl0eF9mZWVfc3Vic2lkeV9tYXhfYW1vdW50GA8gASgEUhV0eEZlZVN1YnNpZHlNYXhBbW91bnQSLgoTYmxvY2tfcmV3YXJkX2Ftb3VudBgQIAEoBFIRYmxvY2tSZXdhcmRBbW91bnQSNQoXYmxvY2tfcmV3YXJkX2xhc3RfYmxvY2sYESABKARSFGJsb2NrUmV3YXJkTGFzdEJsb2NrEi4KE2thcm1hX3Jld2FyZF9hbW91bnQYEiABKARSEWthcm1hUmV3YXJkQW1vdW50EiwKEmthcm1hX3Jld2FyZF9hbGxvYxgTIAEoBFIQa2FybWFSZXdhcmRBbGxvYxI2ChhrYXJtYV9yZXdhcmRfdG9wX25fdXNlcnMYFCABKARSFGthcm1hUmV3YXJkVG9wTlVzZXJzEjYKF3RyZWFzdXJ5X3ByZW1pbnRfYW1vdW50GBUgASgEUhV0cmVhc3VyeVByZW1pbnRBbW91bnQSLgoTdHJlYXN1cnlfYWNjb3VudF9pZBgWIAEoCVIRdHJlYXN1cnlBY2NvdW50SWQSMgoVdHJlYXN1cnlfYWNjb3VudF9uYW1lGBcgASgJUhN0cmVhc3VyeUFjY291bnROYW1lEkEKC2NoYXJfdHJhaXRzGBggAygLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLkNoYXJUcmFpdFIKY2hhclRyYWl0cxJCCgl2ZXJpZmllcnMYGSADKAsyJC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuUGhvbmVWZXJpZmllclIJdmVyaWZpZXJz');
@$core.Deprecated('Use getBlockchainDataRequestDescriptor instead')
const GetBlockchainDataRequest$json = const {
  '1': 'GetBlockchainDataRequest',
};

/// Descriptor for `GetBlockchainDataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getBlockchainDataRequestDescriptor = $convert.base64Decode('ChhHZXRCbG9ja2NoYWluRGF0YVJlcXVlc3Q=');
@$core.Deprecated('Use getBlockchainDataResponseDescriptor instead')
const GetBlockchainDataResponse$json = const {
  '1': 'GetBlockchainDataResponse',
  '2': const [
    const {'1': 'stats', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.BlockchainStats', '10': 'stats'},
  ],
};

/// Descriptor for `GetBlockchainDataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getBlockchainDataResponseDescriptor = $convert.base64Decode('ChlHZXRCbG9ja2NoYWluRGF0YVJlc3BvbnNlEjwKBXN0YXRzGAEgASgLMiYua2FybWFfY29pbi5jb3JlX3R5cGVzLkJsb2NrY2hhaW5TdGF0c1IFc3RhdHM=');
@$core.Deprecated('Use getTransactionsRequestDescriptor instead')
const GetTransactionsRequest$json = const {
  '1': 'GetTransactionsRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
  ],
};

/// Descriptor for `GetTransactionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionsRequestDescriptor = $convert.base64Decode('ChZHZXRUcmFuc2FjdGlvbnNSZXF1ZXN0Ej8KCmFjY291bnRfaWQYASABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQWNjb3VudElkUglhY2NvdW50SWQ=');
@$core.Deprecated('Use getTransactionsResponseDescriptor instead')
const GetTransactionsResponse$json = const {
  '1': 'GetTransactionsResponse',
  '2': const [
    const {'1': 'transactions', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.SignedTransactionWithStatus', '10': 'transactions'},
    const {'1': 'tx_events', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.TransactionEvents', '10': 'txEvents'},
  ],
};

/// Descriptor for `GetTransactionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionsResponseDescriptor = $convert.base64Decode('ChdHZXRUcmFuc2FjdGlvbnNSZXNwb25zZRJWCgx0cmFuc2FjdGlvbnMYASADKAsyMi5rYXJtYV9jb2luLmNvcmVfdHlwZXMuU2lnbmVkVHJhbnNhY3Rpb25XaXRoU3RhdHVzUgx0cmFuc2FjdGlvbnMSRQoJdHhfZXZlbnRzGAIgASgLMigua2FybWFfY29pbi5jb3JlX3R5cGVzLlRyYW5zYWN0aW9uRXZlbnRzUgh0eEV2ZW50cw==');
@$core.Deprecated('Use getTransactionRequestDescriptor instead')
const GetTransactionRequest$json = const {
  '1': 'GetTransactionRequest',
  '2': const [
    const {'1': 'tx_hash', '3': 1, '4': 1, '5': 12, '10': 'txHash'},
  ],
};

/// Descriptor for `GetTransactionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionRequestDescriptor = $convert.base64Decode('ChVHZXRUcmFuc2FjdGlvblJlcXVlc3QSFwoHdHhfaGFzaBgBIAEoDFIGdHhIYXNo');
@$core.Deprecated('Use getTransactionResponseDescriptor instead')
const GetTransactionResponse$json = const {
  '1': 'GetTransactionResponse',
  '2': const [
    const {'1': 'transaction', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.SignedTransactionWithStatus', '10': 'transaction'},
    const {'1': 'tx_events', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.TransactionEvents', '10': 'txEvents'},
  ],
};

/// Descriptor for `GetTransactionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionResponseDescriptor = $convert.base64Decode('ChZHZXRUcmFuc2FjdGlvblJlc3BvbnNlElQKC3RyYW5zYWN0aW9uGAEgASgLMjIua2FybWFfY29pbi5jb3JlX3R5cGVzLlNpZ25lZFRyYW5zYWN0aW9uV2l0aFN0YXR1c1ILdHJhbnNhY3Rpb24SRQoJdHhfZXZlbnRzGAIgASgLMigua2FybWFfY29pbi5jb3JlX3R5cGVzLlRyYW5zYWN0aW9uRXZlbnRzUgh0eEV2ZW50cw==');
@$core.Deprecated('Use getBlockchainEventsRequestDescriptor instead')
const GetBlockchainEventsRequest$json = const {
  '1': 'GetBlockchainEventsRequest',
  '2': const [
    const {'1': 'from_block_height', '3': 1, '4': 1, '5': 4, '10': 'fromBlockHeight'},
    const {'1': 'to_block_height', '3': 2, '4': 1, '5': 4, '10': 'toBlockHeight'},
  ],
};

/// Descriptor for `GetBlockchainEventsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getBlockchainEventsRequestDescriptor = $convert.base64Decode('ChpHZXRCbG9ja2NoYWluRXZlbnRzUmVxdWVzdBIqChFmcm9tX2Jsb2NrX2hlaWdodBgBIAEoBFIPZnJvbUJsb2NrSGVpZ2h0EiYKD3RvX2Jsb2NrX2hlaWdodBgCIAEoBFINdG9CbG9ja0hlaWdodA==');
@$core.Deprecated('Use getBlockchainEventsResponseDescriptor instead')
const GetBlockchainEventsResponse$json = const {
  '1': 'GetBlockchainEventsResponse',
  '2': const [
    const {'1': 'blocks_events', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.BlockEvent', '10': 'blocksEvents'},
  ],
};

/// Descriptor for `GetBlockchainEventsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getBlockchainEventsResponseDescriptor = $convert.base64Decode('ChtHZXRCbG9ja2NoYWluRXZlbnRzUmVzcG9uc2USRgoNYmxvY2tzX2V2ZW50cxgBIAMoCzIhLmthcm1hX2NvaW4uY29yZV90eXBlcy5CbG9ja0V2ZW50UgxibG9ja3NFdmVudHM=');
@$core.Deprecated('Use getBlocksRequestDescriptor instead')
const GetBlocksRequest$json = const {
  '1': 'GetBlocksRequest',
  '2': const [
    const {'1': 'from_block_height', '3': 1, '4': 1, '5': 4, '10': 'fromBlockHeight'},
    const {'1': 'to_block_height', '3': 2, '4': 1, '5': 4, '10': 'toBlockHeight'},
  ],
};

/// Descriptor for `GetBlocksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getBlocksRequestDescriptor = $convert.base64Decode('ChBHZXRCbG9ja3NSZXF1ZXN0EioKEWZyb21fYmxvY2tfaGVpZ2h0GAEgASgEUg9mcm9tQmxvY2tIZWlnaHQSJgoPdG9fYmxvY2tfaGVpZ2h0GAIgASgEUg10b0Jsb2NrSGVpZ2h0');
@$core.Deprecated('Use getBlocksResponseDescriptor instead')
const GetBlocksResponse$json = const {
  '1': 'GetBlocksResponse',
  '2': const [
    const {'1': 'blocks', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.Block', '10': 'blocks'},
  ],
};

/// Descriptor for `GetBlocksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getBlocksResponseDescriptor = $convert.base64Decode('ChFHZXRCbG9ja3NSZXNwb25zZRI0CgZibG9ja3MYASADKAsyHC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQmxvY2tSBmJsb2Nrcw==');
