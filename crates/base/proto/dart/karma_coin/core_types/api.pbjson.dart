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
@$core.Deprecated('Use setCommunityAdminRequestDescriptor instead')
const SetCommunityAdminRequest$json = const {
  '1': 'SetCommunityAdminRequest',
  '2': const [
    const {'1': 'from_account_id', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'fromAccountId'},
    const {'1': 'data', '3': 2, '4': 1, '5': 12, '10': 'data'},
    const {'1': 'signature', '3': 3, '4': 1, '5': 12, '10': 'signature'},
  ],
};

/// Descriptor for `SetCommunityAdminRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setCommunityAdminRequestDescriptor = $convert.base64Decode('ChhTZXRDb21tdW5pdHlBZG1pblJlcXVlc3QSSAoPZnJvbV9hY2NvdW50X2lkGAEgASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLkFjY291bnRJZFINZnJvbUFjY291bnRJZBISCgRkYXRhGAIgASgMUgRkYXRhEhwKCXNpZ25hdHVyZRgDIAEoDFIJc2lnbmF0dXJl');
@$core.Deprecated('Use setCommunityAdminDataDescriptor instead')
const SetCommunityAdminData$json = const {
  '1': 'SetCommunityAdminData',
  '2': const [
    const {'1': 'timestamp', '3': 1, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'target_account_id', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'targetAccountId'},
    const {'1': 'community_id', '3': 3, '4': 1, '5': 13, '10': 'communityId'},
    const {'1': 'admin', '3': 4, '4': 1, '5': 8, '10': 'admin'},
  ],
};

/// Descriptor for `SetCommunityAdminData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setCommunityAdminDataDescriptor = $convert.base64Decode('ChVTZXRDb21tdW5pdHlBZG1pbkRhdGESHAoJdGltZXN0YW1wGAEgASgEUgl0aW1lc3RhbXASTAoRdGFyZ2V0X2FjY291bnRfaWQYAiABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQWNjb3VudElkUg90YXJnZXRBY2NvdW50SWQSIQoMY29tbXVuaXR5X2lkGAMgASgNUgtjb21tdW5pdHlJZBIUCgVhZG1pbhgEIAEoCFIFYWRtaW4=');
@$core.Deprecated('Use setCommunityAdminResponseDescriptor instead')
const SetCommunityAdminResponse$json = const {
  '1': 'SetCommunityAdminResponse',
};

/// Descriptor for `SetCommunityAdminResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setCommunityAdminResponseDescriptor = $convert.base64Decode('ChlTZXRDb21tdW5pdHlBZG1pblJlc3BvbnNl');
@$core.Deprecated('Use getLeaderBoardRequestDescriptor instead')
const GetLeaderBoardRequest$json = const {
  '1': 'GetLeaderBoardRequest',
};

/// Descriptor for `GetLeaderBoardRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getLeaderBoardRequestDescriptor = $convert.base64Decode('ChVHZXRMZWFkZXJCb2FyZFJlcXVlc3Q=');
@$core.Deprecated('Use getContactsRequestDescriptor instead')
const GetContactsRequest$json = const {
  '1': 'GetContactsRequest',
  '2': const [
    const {'1': 'prefix', '3': 1, '4': 1, '5': 9, '10': 'prefix'},
    const {'1': 'community_id', '3': 2, '4': 1, '5': 13, '10': 'communityId'},
  ],
};

/// Descriptor for `GetContactsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getContactsRequestDescriptor = $convert.base64Decode('ChJHZXRDb250YWN0c1JlcXVlc3QSFgoGcHJlZml4GAEgASgJUgZwcmVmaXgSIQoMY29tbXVuaXR5X2lkGAIgASgNUgtjb21tdW5pdHlJZA==');
@$core.Deprecated('Use getContactsResponseDescriptor instead')
const GetContactsResponse$json = const {
  '1': 'GetContactsResponse',
  '2': const [
    const {'1': 'contacts', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.Contact', '10': 'contacts'},
  ],
};

/// Descriptor for `GetContactsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getContactsResponseDescriptor = $convert.base64Decode('ChNHZXRDb250YWN0c1Jlc3BvbnNlEjoKCGNvbnRhY3RzGAEgAygLMh4ua2FybWFfY29pbi5jb3JlX3R5cGVzLkNvbnRhY3RSCGNvbnRhY3Rz');
@$core.Deprecated('Use getLeaderBoardResponseDescriptor instead')
const GetLeaderBoardResponse$json = const {
  '1': 'GetLeaderBoardResponse',
  '2': const [
    const {'1': 'leaderboard_entries', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.LeaderboardEntry', '10': 'leaderboardEntries'},
  ],
};

/// Descriptor for `GetLeaderBoardResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getLeaderBoardResponseDescriptor = $convert.base64Decode('ChZHZXRMZWFkZXJCb2FyZFJlc3BvbnNlElgKE2xlYWRlcmJvYXJkX2VudHJpZXMYASADKAsyJy5rYXJtYV9jb2luLmNvcmVfdHlwZXMuTGVhZGVyYm9hcmRFbnRyeVISbGVhZGVyYm9hcmRFbnRyaWVz');
@$core.Deprecated('Use getTransactionsFromHashesRequestDescriptor instead')
const GetTransactionsFromHashesRequest$json = const {
  '1': 'GetTransactionsFromHashesRequest',
  '2': const [
    const {'1': 'tx_hashes', '3': 1, '4': 3, '5': 12, '10': 'txHashes'},
  ],
};

/// Descriptor for `GetTransactionsFromHashesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionsFromHashesRequestDescriptor = $convert.base64Decode('CiBHZXRUcmFuc2FjdGlvbnNGcm9tSGFzaGVzUmVxdWVzdBIbCgl0eF9oYXNoZXMYASADKAxSCHR4SGFzaGVz');
@$core.Deprecated('Use getTransactionsFromHashesResponseDescriptor instead')
const GetTransactionsFromHashesResponse$json = const {
  '1': 'GetTransactionsFromHashesResponse',
  '2': const [
    const {'1': 'transactions', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.SignedTransactionWithStatus', '10': 'transactions'},
    const {'1': 'tx_events', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.TransactionEvents', '10': 'txEvents'},
  ],
};

/// Descriptor for `GetTransactionsFromHashesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionsFromHashesResponseDescriptor = $convert.base64Decode('CiFHZXRUcmFuc2FjdGlvbnNGcm9tSGFzaGVzUmVzcG9uc2USVgoMdHJhbnNhY3Rpb25zGAEgAygLMjIua2FybWFfY29pbi5jb3JlX3R5cGVzLlNpZ25lZFRyYW5zYWN0aW9uV2l0aFN0YXR1c1IMdHJhbnNhY3Rpb25zEkUKCXR4X2V2ZW50cxgCIAEoCzIoLmthcm1hX2NvaW4uY29yZV90eXBlcy5UcmFuc2FjdGlvbkV2ZW50c1IIdHhFdmVudHM=');
@$core.Deprecated('Use getAllUsersRequestDescriptor instead')
const GetAllUsersRequest$json = const {
  '1': 'GetAllUsersRequest',
  '2': const [
    const {'1': 'community_id', '3': 1, '4': 1, '5': 13, '10': 'communityId'},
  ],
};

/// Descriptor for `GetAllUsersRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAllUsersRequestDescriptor = $convert.base64Decode('ChJHZXRBbGxVc2Vyc1JlcXVlc3QSIQoMY29tbXVuaXR5X2lkGAEgASgNUgtjb21tdW5pdHlJZA==');
@$core.Deprecated('Use getAllUsersResponseDescriptor instead')
const GetAllUsersResponse$json = const {
  '1': 'GetAllUsersResponse',
  '2': const [
    const {'1': 'users', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.User', '10': 'users'},
  ],
};

/// Descriptor for `GetAllUsersResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAllUsersResponseDescriptor = $convert.base64Decode('ChNHZXRBbGxVc2Vyc1Jlc3BvbnNlEjEKBXVzZXJzGAEgAygLMhsua2FybWFfY29pbi5jb3JlX3R5cGVzLlVzZXJSBXVzZXJz');
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
    const {'1': 'genesis_data', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.GenesisData', '10': 'genesisData'},
  ],
};

/// Descriptor for `GetGenesisDataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getGenesisDataResponseDescriptor = $convert.base64Decode('ChZHZXRHZW5lc2lzRGF0YVJlc3BvbnNlEkUKDGdlbmVzaXNfZGF0YRgBIAEoCzIiLmthcm1hX2NvaW4uY29yZV90eXBlcy5HZW5lc2lzRGF0YVILZ2VuZXNpc0RhdGE=');
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
