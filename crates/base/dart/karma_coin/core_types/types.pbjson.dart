///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/types.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use keySchemeDescriptor instead')
const KeyScheme$json = const {
  '1': 'KeyScheme',
  '2': const [
    const {'1': 'KEY_SCHEME_ED25519', '2': 0},
  ],
};

/// Descriptor for `KeyScheme`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List keySchemeDescriptor = $convert.base64Decode('CglLZXlTY2hlbWUSFgoSS0VZX1NDSEVNRV9FRDI1NTE5EAA=');
@$core.Deprecated('Use transactionTypeDescriptor instead')
const TransactionType$json = const {
  '1': 'TransactionType',
  '2': const [
    const {'1': 'TRANSACTION_TYPE_PAYMENT_V1', '2': 0},
    const {'1': 'TRANSACTION_TYPE_NEW_USER_V1', '2': 1},
    const {'1': 'TRANSACTION_TYPE_UPDATE_USER_V1', '2': 2},
  ],
};

/// Descriptor for `TransactionType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List transactionTypeDescriptor = $convert.base64Decode('Cg9UcmFuc2FjdGlvblR5cGUSHwobVFJBTlNBQ1RJT05fVFlQRV9QQVlNRU5UX1YxEAASIAocVFJBTlNBQ1RJT05fVFlQRV9ORVdfVVNFUl9WMRABEiMKH1RSQU5TQUNUSU9OX1RZUEVfVVBEQVRFX1VTRVJfVjEQAg==');
@$core.Deprecated('Use verificationResultDescriptor instead')
const VerificationResult$json = const {
  '1': 'VerificationResult',
  '2': const [
    const {'1': 'VERIFICATION_RESULT_UNSPECIFIED', '2': 0},
    const {'1': 'VERIFICATION_RESULT_USER_NAME_TAKEN', '2': 1},
    const {'1': 'VERIFICATION_RESULT_VERIFIED', '2': 2},
    const {'1': 'VERIFICATION_RESULT_UNVERIFIED', '2': 3},
    const {'1': 'VERIFICATION_RESULT_MISSING_DATA', '2': 4},
    const {'1': 'VERIFICATION_RESULT_INVALID_SIGNATURE', '2': 5},
    const {'1': 'VERIFICATION_RESULT_ACCOUNT_MISMATCH', '2': 6},
  ],
};

/// Descriptor for `VerificationResult`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List verificationResultDescriptor = $convert.base64Decode('ChJWZXJpZmljYXRpb25SZXN1bHQSIwofVkVSSUZJQ0FUSU9OX1JFU1VMVF9VTlNQRUNJRklFRBAAEicKI1ZFUklGSUNBVElPTl9SRVNVTFRfVVNFUl9OQU1FX1RBS0VOEAESIAocVkVSSUZJQ0FUSU9OX1JFU1VMVF9WRVJJRklFRBACEiIKHlZFUklGSUNBVElPTl9SRVNVTFRfVU5WRVJJRklFRBADEiQKIFZFUklGSUNBVElPTl9SRVNVTFRfTUlTU0lOR19EQVRBEAQSKQolVkVSSUZJQ0FUSU9OX1JFU1VMVF9JTlZBTElEX1NJR05BVFVSRRAFEigKJFZFUklGSUNBVElPTl9SRVNVTFRfQUNDT1VOVF9NSVNNQVRDSBAG');
@$core.Deprecated('Use transactionStatusDescriptor instead')
const TransactionStatus$json = const {
  '1': 'TransactionStatus',
  '2': const [
    const {'1': 'TRANSACTION_STATUS_UNKNOWN', '2': 0},
    const {'1': 'TRANSACTION_STATUS_NOT_SUBMITTED', '2': 1},
    const {'1': 'TRANSACTION_STATUS_SUBMITTED', '2': 2},
    const {'1': 'TRANSACTION_STATUS_REJECTED', '2': 3},
    const {'1': 'TRANSACTION_STATUS_ON_CHAIN', '2': 4},
  ],
};

/// Descriptor for `TransactionStatus`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List transactionStatusDescriptor = $convert.base64Decode('ChFUcmFuc2FjdGlvblN0YXR1cxIeChpUUkFOU0FDVElPTl9TVEFUVVNfVU5LTk9XThAAEiQKIFRSQU5TQUNUSU9OX1NUQVRVU19OT1RfU1VCTUlUVEVEEAESIAocVFJBTlNBQ1RJT05fU1RBVFVTX1NVQk1JVFRFRBACEh8KG1RSQU5TQUNUSU9OX1NUQVRVU19SRUpFQ1RFRBADEh8KG1RSQU5TQUNUSU9OX1NUQVRVU19PTl9DSEFJThAE');
@$core.Deprecated('Use feeTypeDescriptor instead')
const FeeType$json = const {
  '1': 'FeeType',
  '2': const [
    const {'1': 'FEE_TYPE_MINT', '2': 0},
    const {'1': 'FEE_TYPE_USER', '2': 1},
  ],
};

/// Descriptor for `FeeType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List feeTypeDescriptor = $convert.base64Decode('CgdGZWVUeXBlEhEKDUZFRV9UWVBFX01JTlQQABIRCg1GRUVfVFlQRV9VU0VSEAE=');
@$core.Deprecated('Use executionResultDescriptor instead')
const ExecutionResult$json = const {
  '1': 'ExecutionResult',
  '2': const [
    const {'1': 'EXECUTION_RESULT_EXECUTED', '2': 0},
    const {'1': 'EXECUTION_RESULT_INVALID', '2': 1},
  ],
};

/// Descriptor for `ExecutionResult`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List executionResultDescriptor = $convert.base64Decode('Cg9FeGVjdXRpb25SZXN1bHQSHQoZRVhFQ1VUSU9OX1JFU1VMVF9FWEVDVVRFRBAAEhwKGEVYRUNVVElPTl9SRVNVTFRfSU5WQUxJRBAB');
@$core.Deprecated('Use executionInfoDescriptor instead')
const ExecutionInfo$json = const {
  '1': 'ExecutionInfo',
  '2': const [
    const {'1': 'EXECUTION_INFO_UNKNOWN', '2': 0},
    const {'1': 'EXECUTION_INFO_NICKNAME_UPDATED', '2': 1},
    const {'1': 'EXECUTION_INFO_NICKNAME_NOT_AVAILABLE', '2': 2},
    const {'1': 'EXECUTION_INFO_NICKNAME_INVALID', '2': 3},
    const {'1': 'EXECUTION_INFO_NUMBER_UPDATED', '2': 4},
    const {'1': 'EXECUTION_INFO_ACCOUNT_CREATED', '2': 5},
    const {'1': 'EXECUTION_INFO_PAYMENT_CONFIRMED', '2': 6},
    const {'1': 'EXECUTION_INFO_INVALID_DATA', '2': 7},
    const {'1': 'EXECUTION_INFO_ACCOUNT_ALREADY_EXISTS', '2': 8},
    const {'1': 'EXECUTION_INFO_TX_FEE_TOO_LOW', '2': 9},
    const {'1': 'EXECUTION_INFO_INTERNAL_NODE_ERROR', '2': 10},
  ],
};

/// Descriptor for `ExecutionInfo`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List executionInfoDescriptor = $convert.base64Decode('Cg1FeGVjdXRpb25JbmZvEhoKFkVYRUNVVElPTl9JTkZPX1VOS05PV04QABIjCh9FWEVDVVRJT05fSU5GT19OSUNLTkFNRV9VUERBVEVEEAESKQolRVhFQ1VUSU9OX0lORk9fTklDS05BTUVfTk9UX0FWQUlMQUJMRRACEiMKH0VYRUNVVElPTl9JTkZPX05JQ0tOQU1FX0lOVkFMSUQQAxIhCh1FWEVDVVRJT05fSU5GT19OVU1CRVJfVVBEQVRFRBAEEiIKHkVYRUNVVElPTl9JTkZPX0FDQ09VTlRfQ1JFQVRFRBAFEiQKIEVYRUNVVElPTl9JTkZPX1BBWU1FTlRfQ09ORklSTUVEEAYSHwobRVhFQ1VUSU9OX0lORk9fSU5WQUxJRF9EQVRBEAcSKQolRVhFQ1VUSU9OX0lORk9fQUNDT1VOVF9BTFJFQURZX0VYSVNUUxAIEiEKHUVYRUNVVElPTl9JTkZPX1RYX0ZFRV9UT09fTE9XEAkSJgoiRVhFQ1VUSU9OX0lORk9fSU5URVJOQUxfTk9ERV9FUlJPUhAK');
@$core.Deprecated('Use accountIdDescriptor instead')
const AccountId$json = const {
  '1': 'AccountId',
  '2': const [
    const {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
  ],
};

/// Descriptor for `AccountId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountIdDescriptor = $convert.base64Decode('CglBY2NvdW50SWQSEgoEZGF0YRgBIAEoDFIEZGF0YQ==');
@$core.Deprecated('Use balanceDescriptor instead')
const Balance$json = const {
  '1': 'Balance',
  '2': const [
    const {'1': 'free', '3': 1, '4': 1, '5': 4, '10': 'free'},
    const {'1': 'reserved', '3': 2, '4': 1, '5': 4, '10': 'reserved'},
    const {'1': 'misc_frozen', '3': 3, '4': 1, '5': 4, '10': 'miscFrozen'},
    const {'1': 'fee_frozen', '3': 4, '4': 1, '5': 4, '10': 'feeFrozen'},
  ],
};

/// Descriptor for `Balance`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List balanceDescriptor = $convert.base64Decode('CgdCYWxhbmNlEhIKBGZyZWUYASABKARSBGZyZWUSGgoIcmVzZXJ2ZWQYAiABKARSCHJlc2VydmVkEh8KC21pc2NfZnJvemVuGAMgASgEUgptaXNjRnJvemVuEh0KCmZlZV9mcm96ZW4YBCABKARSCWZlZUZyb3plbg==');
@$core.Deprecated('Use publicKeyDescriptor instead')
const PublicKey$json = const {
  '1': 'PublicKey',
  '2': const [
    const {'1': 'key', '3': 1, '4': 1, '5': 12, '10': 'key'},
  ],
};

/// Descriptor for `PublicKey`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List publicKeyDescriptor = $convert.base64Decode('CglQdWJsaWNLZXkSEAoDa2V5GAEgASgMUgNrZXk=');
@$core.Deprecated('Use privateKeyDescriptor instead')
const PrivateKey$json = const {
  '1': 'PrivateKey',
  '2': const [
    const {'1': 'key', '3': 1, '4': 1, '5': 12, '10': 'key'},
  ],
};

/// Descriptor for `PrivateKey`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List privateKeyDescriptor = $convert.base64Decode('CgpQcml2YXRlS2V5EhAKA2tleRgBIAEoDFIDa2V5');
@$core.Deprecated('Use preKeyDescriptor instead')
const PreKey$json = const {
  '1': 'PreKey',
  '2': const [
    const {'1': 'pub_key', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.PublicKey', '10': 'pubKey'},
    const {'1': 'id', '3': 2, '4': 1, '5': 13, '10': 'id'},
    const {'1': 'scheme', '3': 3, '4': 1, '5': 14, '6': '.karma_coin.core_types.KeyScheme', '10': 'scheme'},
  ],
};

/// Descriptor for `PreKey`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List preKeyDescriptor = $convert.base64Decode('CgZQcmVLZXkSOQoHcHViX2tleRgBIAEoCzIgLmthcm1hX2NvaW4uY29yZV90eXBlcy5QdWJsaWNLZXlSBnB1YktleRIOCgJpZBgCIAEoDVICaWQSOAoGc2NoZW1lGAMgASgOMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLktleVNjaGVtZVIGc2NoZW1l');
@$core.Deprecated('Use keyPairDescriptor instead')
const KeyPair$json = const {
  '1': 'KeyPair',
  '2': const [
    const {'1': 'private_key', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.PrivateKey', '10': 'privateKey'},
    const {'1': 'public_key', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.PublicKey', '10': 'publicKey'},
    const {'1': 'scheme', '3': 3, '4': 1, '5': 14, '6': '.karma_coin.core_types.KeyScheme', '10': 'scheme'},
  ],
};

/// Descriptor for `KeyPair`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List keyPairDescriptor = $convert.base64Decode('CgdLZXlQYWlyEkIKC3ByaXZhdGVfa2V5GAEgASgLMiEua2FybWFfY29pbi5jb3JlX3R5cGVzLlByaXZhdGVLZXlSCnByaXZhdGVLZXkSPwoKcHVibGljX2tleRgCIAEoCzIgLmthcm1hX2NvaW4uY29yZV90eXBlcy5QdWJsaWNLZXlSCXB1YmxpY0tleRI4CgZzY2hlbWUYAyABKA4yIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuS2V5U2NoZW1lUgZzY2hlbWU=');
@$core.Deprecated('Use signatureDescriptor instead')
const Signature$json = const {
  '1': 'Signature',
  '2': const [
    const {'1': 'scheme', '3': 1, '4': 1, '5': 14, '6': '.karma_coin.core_types.KeyScheme', '10': 'scheme'},
    const {'1': 'signature', '3': 2, '4': 1, '5': 12, '10': 'signature'},
  ],
};

/// Descriptor for `Signature`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signatureDescriptor = $convert.base64Decode('CglTaWduYXR1cmUSOAoGc2NoZW1lGAEgASgOMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLktleVNjaGVtZVIGc2NoZW1lEhwKCXNpZ25hdHVyZRgCIAEoDFIJc2lnbmF0dXJl');
@$core.Deprecated('Use mobileNumberDescriptor instead')
const MobileNumber$json = const {
  '1': 'MobileNumber',
  '2': const [
    const {'1': 'number', '3': 1, '4': 1, '5': 9, '10': 'number'},
  ],
};

/// Descriptor for `MobileNumber`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List mobileNumberDescriptor = $convert.base64Decode('CgxNb2JpbGVOdW1iZXISFgoGbnVtYmVyGAEgASgJUgZudW1iZXI=');
@$core.Deprecated('Use userDescriptor instead')
const User$json = const {
  '1': 'User',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
    const {'1': 'nonce', '3': 2, '4': 1, '5': 4, '10': 'nonce'},
    const {'1': 'user_name', '3': 3, '4': 1, '5': 9, '10': 'userName'},
    const {'1': 'mobile_number', '3': 4, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'mobileNumber'},
    const {'1': 'balance', '3': 5, '4': 1, '5': 4, '10': 'balance'},
    const {'1': 'trait_scores', '3': 6, '4': 3, '5': 11, '6': '.karma_coin.core_types.TraitScore', '10': 'traitScores'},
    const {'1': 'pre_keys', '3': 7, '4': 3, '5': 11, '6': '.karma_coin.core_types.PreKey', '10': 'preKeys'},
    const {'1': 'karma_score', '3': 8, '4': 1, '5': 13, '10': 'karmaScore'},
  ],
};

/// Descriptor for `User`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userDescriptor = $convert.base64Decode('CgRVc2VyEj8KCmFjY291bnRfaWQYASABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQWNjb3VudElkUglhY2NvdW50SWQSFAoFbm9uY2UYAiABKARSBW5vbmNlEhsKCXVzZXJfbmFtZRgDIAEoCVIIdXNlck5hbWUSSAoNbW9iaWxlX251bWJlchgEIAEoCzIjLmthcm1hX2NvaW4uY29yZV90eXBlcy5Nb2JpbGVOdW1iZXJSDG1vYmlsZU51bWJlchIYCgdiYWxhbmNlGAUgASgEUgdiYWxhbmNlEkQKDHRyYWl0X3Njb3JlcxgGIAMoCzIhLmthcm1hX2NvaW4uY29yZV90eXBlcy5UcmFpdFNjb3JlUgt0cmFpdFNjb3JlcxI4CghwcmVfa2V5cxgHIAMoCzIdLmthcm1hX2NvaW4uY29yZV90eXBlcy5QcmVLZXlSB3ByZUtleXMSHwoLa2FybWFfc2NvcmUYCCABKA1SCmthcm1hU2NvcmU=');
@$core.Deprecated('Use phoneVerifierDescriptor instead')
const PhoneVerifier$json = const {
  '1': 'PhoneVerifier',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
  ],
};

/// Descriptor for `PhoneVerifier`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List phoneVerifierDescriptor = $convert.base64Decode('Cg1QaG9uZVZlcmlmaWVyEj8KCmFjY291bnRfaWQYASABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQWNjb3VudElkUglhY2NvdW50SWQSEgoEbmFtZRgCIAEoCVIEbmFtZQ==');
@$core.Deprecated('Use blockDescriptor instead')
const Block$json = const {
  '1': 'Block',
  '2': const [
    const {'1': 'time', '3': 1, '4': 1, '5': 4, '10': 'time'},
    const {'1': 'author', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'author'},
    const {'1': 'height', '3': 3, '4': 1, '5': 4, '10': 'height'},
    const {'1': 'transactions_hashes', '3': 4, '4': 3, '5': 12, '10': 'transactionsHashes'},
    const {'1': 'fees', '3': 5, '4': 1, '5': 4, '10': 'fees'},
    const {'1': 'prev_block_digest', '3': 6, '4': 1, '5': 12, '10': 'prevBlockDigest'},
    const {'1': 'signature', '3': 7, '4': 1, '5': 11, '6': '.karma_coin.core_types.Signature', '10': 'signature'},
    const {'1': 'reward', '3': 8, '4': 1, '5': 4, '10': 'reward'},
    const {'1': 'minted', '3': 9, '4': 1, '5': 4, '10': 'minted'},
    const {'1': 'digest', '3': 10, '4': 1, '5': 12, '10': 'digest'},
  ],
};

/// Descriptor for `Block`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List blockDescriptor = $convert.base64Decode('CgVCbG9jaxISCgR0aW1lGAEgASgEUgR0aW1lEjgKBmF1dGhvchgCIAEoCzIgLmthcm1hX2NvaW4uY29yZV90eXBlcy5BY2NvdW50SWRSBmF1dGhvchIWCgZoZWlnaHQYAyABKARSBmhlaWdodBIvChN0cmFuc2FjdGlvbnNfaGFzaGVzGAQgAygMUhJ0cmFuc2FjdGlvbnNIYXNoZXMSEgoEZmVlcxgFIAEoBFIEZmVlcxIqChFwcmV2X2Jsb2NrX2RpZ2VzdBgGIAEoDFIPcHJldkJsb2NrRGlnZXN0Ej4KCXNpZ25hdHVyZRgHIAEoCzIgLmthcm1hX2NvaW4uY29yZV90eXBlcy5TaWduYXR1cmVSCXNpZ25hdHVyZRIWCgZyZXdhcmQYCCABKARSBnJld2FyZBIWCgZtaW50ZWQYCSABKARSBm1pbnRlZBIWCgZkaWdlc3QYCiABKAxSBmRpZ2VzdA==');
@$core.Deprecated('Use charTraitDescriptor instead')
const CharTrait$json = const {
  '1': 'CharTrait',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 13, '10': 'id'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'emoji', '3': 3, '4': 1, '5': 9, '10': 'emoji'},
  ],
};

/// Descriptor for `CharTrait`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List charTraitDescriptor = $convert.base64Decode('CglDaGFyVHJhaXQSDgoCaWQYASABKA1SAmlkEhIKBG5hbWUYAiABKAlSBG5hbWUSFAoFZW1vamkYAyABKAlSBWVtb2pp');
@$core.Deprecated('Use traitScoreDescriptor instead')
const TraitScore$json = const {
  '1': 'TraitScore',
  '2': const [
    const {'1': 'trait_id', '3': 1, '4': 1, '5': 13, '10': 'traitId'},
    const {'1': 'score', '3': 2, '4': 1, '5': 13, '10': 'score'},
  ],
};

/// Descriptor for `TraitScore`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List traitScoreDescriptor = $convert.base64Decode('CgpUcmFpdFNjb3JlEhkKCHRyYWl0X2lkGAEgASgNUgd0cmFpdElkEhQKBXNjb3JlGAIgASgNUgVzY29yZQ==');
@$core.Deprecated('Use updateUserTransactionV1Descriptor instead')
const UpdateUserTransactionV1$json = const {
  '1': 'UpdateUserTransactionV1',
  '2': const [
    const {'1': 'nickname', '3': 1, '4': 1, '5': 9, '10': 'nickname'},
    const {'1': 'mobile_number', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'mobileNumber'},
    const {'1': 'user_verification_data', '3': 3, '4': 1, '5': 11, '6': '.karma_coin.core_types.UserVerificationData', '10': 'userVerificationData'},
  ],
};

/// Descriptor for `UpdateUserTransactionV1`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List updateUserTransactionV1Descriptor = $convert.base64Decode('ChdVcGRhdGVVc2VyVHJhbnNhY3Rpb25WMRIaCghuaWNrbmFtZRgBIAEoCVIIbmlja25hbWUSSAoNbW9iaWxlX251bWJlchgCIAEoCzIjLmthcm1hX2NvaW4uY29yZV90eXBlcy5Nb2JpbGVOdW1iZXJSDG1vYmlsZU51bWJlchJhChZ1c2VyX3ZlcmlmaWNhdGlvbl9kYXRhGAMgASgLMisua2FybWFfY29pbi5jb3JlX3R5cGVzLlVzZXJWZXJpZmljYXRpb25EYXRhUhR1c2VyVmVyaWZpY2F0aW9uRGF0YQ==');
@$core.Deprecated('Use paymentTransactionV1Descriptor instead')
const PaymentTransactionV1$json = const {
  '1': 'PaymentTransactionV1',
  '2': const [
    const {'1': 'to', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'to'},
    const {'1': 'amount', '3': 2, '4': 1, '5': 4, '10': 'amount'},
    const {'1': 'char_trait_id', '3': 3, '4': 1, '5': 13, '10': 'charTraitId'},
  ],
};

/// Descriptor for `PaymentTransactionV1`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List paymentTransactionV1Descriptor = $convert.base64Decode('ChRQYXltZW50VHJhbnNhY3Rpb25WMRIzCgJ0bxgBIAEoCzIjLmthcm1hX2NvaW4uY29yZV90eXBlcy5Nb2JpbGVOdW1iZXJSAnRvEhYKBmFtb3VudBgCIAEoBFIGYW1vdW50EiIKDWNoYXJfdHJhaXRfaWQYAyABKA1SC2NoYXJUcmFpdElk');
@$core.Deprecated('Use userVerificationDataDescriptor instead')
const UserVerificationData$json = const {
  '1': 'UserVerificationData',
  '2': const [
    const {'1': 'verifier_account_id', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'verifierAccountId'},
    const {'1': 'timestamp', '3': 2, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'verification_result', '3': 3, '4': 1, '5': 14, '6': '.karma_coin.core_types.VerificationResult', '10': 'verificationResult'},
    const {'1': 'account_id', '3': 4, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
    const {'1': 'mobile_number', '3': 5, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'mobileNumber'},
    const {'1': 'requested_user_name', '3': 7, '4': 1, '5': 9, '10': 'requestedUserName'},
    const {'1': 'signature', '3': 8, '4': 1, '5': 11, '6': '.karma_coin.core_types.Signature', '10': 'signature'},
  ],
};

/// Descriptor for `UserVerificationData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List userVerificationDataDescriptor = $convert.base64Decode('ChRVc2VyVmVyaWZpY2F0aW9uRGF0YRJQChN2ZXJpZmllcl9hY2NvdW50X2lkGAEgASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLkFjY291bnRJZFIRdmVyaWZpZXJBY2NvdW50SWQSHAoJdGltZXN0YW1wGAIgASgEUgl0aW1lc3RhbXASWgoTdmVyaWZpY2F0aW9uX3Jlc3VsdBgDIAEoDjIpLmthcm1hX2NvaW4uY29yZV90eXBlcy5WZXJpZmljYXRpb25SZXN1bHRSEnZlcmlmaWNhdGlvblJlc3VsdBI/CgphY2NvdW50X2lkGAQgASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLkFjY291bnRJZFIJYWNjb3VudElkEkgKDW1vYmlsZV9udW1iZXIYBSABKAsyIy5rYXJtYV9jb2luLmNvcmVfdHlwZXMuTW9iaWxlTnVtYmVyUgxtb2JpbGVOdW1iZXISLgoTcmVxdWVzdGVkX3VzZXJfbmFtZRgHIAEoCVIRcmVxdWVzdGVkVXNlck5hbWUSPgoJc2lnbmF0dXJlGAggASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLlNpZ25hdHVyZVIJc2lnbmF0dXJl');
@$core.Deprecated('Use newUserTransactionV1Descriptor instead')
const NewUserTransactionV1$json = const {
  '1': 'NewUserTransactionV1',
  '2': const [
    const {'1': 'verify_number_response', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.UserVerificationData', '10': 'verifyNumberResponse'},
  ],
};

/// Descriptor for `NewUserTransactionV1`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newUserTransactionV1Descriptor = $convert.base64Decode('ChROZXdVc2VyVHJhbnNhY3Rpb25WMRJhChZ2ZXJpZnlfbnVtYmVyX3Jlc3BvbnNlGAEgASgLMisua2FybWFfY29pbi5jb3JlX3R5cGVzLlVzZXJWZXJpZmljYXRpb25EYXRhUhR2ZXJpZnlOdW1iZXJSZXNwb25zZQ==');
@$core.Deprecated('Use transactionDataDescriptor instead')
const TransactionData$json = const {
  '1': 'TransactionData',
  '2': const [
    const {'1': 'transaction_data', '3': 1, '4': 1, '5': 12, '10': 'transactionData'},
    const {'1': 'transaction_type', '3': 2, '4': 1, '5': 14, '6': '.karma_coin.core_types.TransactionType', '10': 'transactionType'},
  ],
};

/// Descriptor for `TransactionData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionDataDescriptor = $convert.base64Decode('Cg9UcmFuc2FjdGlvbkRhdGESKQoQdHJhbnNhY3Rpb25fZGF0YRgBIAEoDFIPdHJhbnNhY3Rpb25EYXRhElEKEHRyYW5zYWN0aW9uX3R5cGUYAiABKA4yJi5rYXJtYV9jb2luLmNvcmVfdHlwZXMuVHJhbnNhY3Rpb25UeXBlUg90cmFuc2FjdGlvblR5cGU=');
@$core.Deprecated('Use signedTransactionDescriptor instead')
const SignedTransaction$json = const {
  '1': 'SignedTransaction',
  '2': const [
    const {'1': 'signer', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'signer'},
    const {'1': 'timestamp', '3': 2, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'nonce', '3': 3, '4': 1, '5': 4, '10': 'nonce'},
    const {'1': 'fee', '3': 4, '4': 1, '5': 4, '10': 'fee'},
    const {'1': 'transaction_Data', '3': 5, '4': 1, '5': 11, '6': '.karma_coin.core_types.TransactionData', '10': 'transactionData'},
    const {'1': 'net_id', '3': 6, '4': 1, '5': 13, '10': 'netId'},
    const {'1': 'signature', '3': 7, '4': 1, '5': 11, '6': '.karma_coin.core_types.Signature', '10': 'signature'},
  ],
};

/// Descriptor for `SignedTransaction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signedTransactionDescriptor = $convert.base64Decode('ChFTaWduZWRUcmFuc2FjdGlvbhI4CgZzaWduZXIYASABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQWNjb3VudElkUgZzaWduZXISHAoJdGltZXN0YW1wGAIgASgEUgl0aW1lc3RhbXASFAoFbm9uY2UYAyABKARSBW5vbmNlEhAKA2ZlZRgEIAEoBFIDZmVlElEKEHRyYW5zYWN0aW9uX0RhdGEYBSABKAsyJi5rYXJtYV9jb2luLmNvcmVfdHlwZXMuVHJhbnNhY3Rpb25EYXRhUg90cmFuc2FjdGlvbkRhdGESFQoGbmV0X2lkGAYgASgNUgVuZXRJZBI+CglzaWduYXR1cmUYByABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuU2lnbmF0dXJlUglzaWduYXR1cmU=');
@$core.Deprecated('Use signedTransactionsHashesDescriptor instead')
const SignedTransactionsHashes$json = const {
  '1': 'SignedTransactionsHashes',
  '2': const [
    const {'1': 'hashes', '3': 1, '4': 3, '5': 12, '10': 'hashes'},
  ],
};

/// Descriptor for `SignedTransactionsHashes`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signedTransactionsHashesDescriptor = $convert.base64Decode('ChhTaWduZWRUcmFuc2FjdGlvbnNIYXNoZXMSFgoGaGFzaGVzGAEgAygMUgZoYXNoZXM=');
@$core.Deprecated('Use memPoolDescriptor instead')
const MemPool$json = const {
  '1': 'MemPool',
  '2': const [
    const {'1': 'transactions', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.SignedTransaction', '10': 'transactions'},
  ],
};

/// Descriptor for `MemPool`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List memPoolDescriptor = $convert.base64Decode('CgdNZW1Qb29sEkwKDHRyYW5zYWN0aW9ucxgBIAMoCzIoLmthcm1hX2NvaW4uY29yZV90eXBlcy5TaWduZWRUcmFuc2FjdGlvblIMdHJhbnNhY3Rpb25z');
@$core.Deprecated('Use signedTransactionWithStatusDescriptor instead')
const SignedTransactionWithStatus$json = const {
  '1': 'SignedTransactionWithStatus',
  '2': const [
    const {'1': 'transaction', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.SignedTransaction', '10': 'transaction'},
    const {'1': 'status', '3': 2, '4': 1, '5': 14, '6': '.karma_coin.core_types.TransactionStatus', '10': 'status'},
  ],
};

/// Descriptor for `SignedTransactionWithStatus`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signedTransactionWithStatusDescriptor = $convert.base64Decode('ChtTaWduZWRUcmFuc2FjdGlvbldpdGhTdGF0dXMSSgoLdHJhbnNhY3Rpb24YASABKAsyKC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuU2lnbmVkVHJhbnNhY3Rpb25SC3RyYW5zYWN0aW9uEkAKBnN0YXR1cxgCIAEoDjIoLmthcm1hX2NvaW4uY29yZV90eXBlcy5UcmFuc2FjdGlvblN0YXR1c1IGc3RhdHVz');
@$core.Deprecated('Use transactionEventDescriptor instead')
const TransactionEvent$json = const {
  '1': 'TransactionEvent',
  '2': const [
    const {'1': 'timestamp', '3': 1, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'height', '3': 2, '4': 1, '5': 4, '10': 'height'},
    const {'1': 'transaction', '3': 3, '4': 1, '5': 11, '6': '.karma_coin.core_types.SignedTransaction', '10': 'transaction'},
    const {'1': 'transaction_hash', '3': 4, '4': 1, '5': 12, '10': 'transactionHash'},
    const {'1': 'result', '3': 5, '4': 1, '5': 14, '6': '.karma_coin.core_types.ExecutionResult', '10': 'result'},
    const {'1': 'info', '3': 6, '4': 1, '5': 14, '6': '.karma_coin.core_types.ExecutionInfo', '10': 'info'},
    const {'1': 'error_message', '3': 7, '4': 1, '5': 9, '10': 'errorMessage'},
    const {'1': 'fee_type', '3': 8, '4': 1, '5': 14, '6': '.karma_coin.core_types.FeeType', '10': 'feeType'},
    const {'1': 'signup_reward', '3': 9, '4': 1, '5': 4, '10': 'signupReward'},
    const {'1': 'referral_reward', '3': 10, '4': 1, '5': 4, '10': 'referralReward'},
    const {'1': 'fee', '3': 11, '4': 1, '5': 4, '10': 'fee'},
  ],
};

/// Descriptor for `TransactionEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionEventDescriptor = $convert.base64Decode('ChBUcmFuc2FjdGlvbkV2ZW50EhwKCXRpbWVzdGFtcBgBIAEoBFIJdGltZXN0YW1wEhYKBmhlaWdodBgCIAEoBFIGaGVpZ2h0EkoKC3RyYW5zYWN0aW9uGAMgASgLMigua2FybWFfY29pbi5jb3JlX3R5cGVzLlNpZ25lZFRyYW5zYWN0aW9uUgt0cmFuc2FjdGlvbhIpChB0cmFuc2FjdGlvbl9oYXNoGAQgASgMUg90cmFuc2FjdGlvbkhhc2gSPgoGcmVzdWx0GAUgASgOMiYua2FybWFfY29pbi5jb3JlX3R5cGVzLkV4ZWN1dGlvblJlc3VsdFIGcmVzdWx0EjgKBGluZm8YBiABKA4yJC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuRXhlY3V0aW9uSW5mb1IEaW5mbxIjCg1lcnJvcl9tZXNzYWdlGAcgASgJUgxlcnJvck1lc3NhZ2USOQoIZmVlX3R5cGUYCCABKA4yHi5rYXJtYV9jb2luLmNvcmVfdHlwZXMuRmVlVHlwZVIHZmVlVHlwZRIjCg1zaWdudXBfcmV3YXJkGAkgASgEUgxzaWdudXBSZXdhcmQSJwoPcmVmZXJyYWxfcmV3YXJkGAogASgEUg5yZWZlcnJhbFJld2FyZBIQCgNmZWUYCyABKARSA2ZlZQ==');
@$core.Deprecated('Use transactionEventsDescriptor instead')
const TransactionEvents$json = const {
  '1': 'TransactionEvents',
  '2': const [
    const {'1': 'events', '3': 1, '4': 3, '5': 11, '6': '.karma_coin.core_types.TransactionEvent', '10': 'events'},
  ],
};

/// Descriptor for `TransactionEvents`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionEventsDescriptor = $convert.base64Decode('ChFUcmFuc2FjdGlvbkV2ZW50cxI/CgZldmVudHMYASADKAsyJy5rYXJtYV9jb2luLmNvcmVfdHlwZXMuVHJhbnNhY3Rpb25FdmVudFIGZXZlbnRz');
@$core.Deprecated('Use blockchainStatsDescriptor instead')
const BlockchainStats$json = const {
  '1': 'BlockchainStats',
  '2': const [
    const {'1': 'last_block_time', '3': 1, '4': 1, '5': 4, '10': 'lastBlockTime'},
    const {'1': 'tip_height', '3': 2, '4': 1, '5': 4, '10': 'tipHeight'},
    const {'1': 'transactions_count', '3': 3, '4': 1, '5': 4, '10': 'transactionsCount'},
    const {'1': 'payments_transactions_count', '3': 4, '4': 1, '5': 4, '10': 'paymentsTransactionsCount'},
    const {'1': 'users_count', '3': 5, '4': 1, '5': 4, '10': 'usersCount'},
    const {'1': 'fees_amount', '3': 6, '4': 1, '5': 4, '10': 'feesAmount'},
    const {'1': 'minted_amount', '3': 7, '4': 1, '5': 4, '10': 'mintedAmount'},
    const {'1': 'circulation', '3': 8, '4': 1, '5': 4, '10': 'circulation'},
    const {'1': 'fee_subs_count', '3': 9, '4': 1, '5': 4, '10': 'feeSubsCount'},
    const {'1': 'fee_subs_amount', '3': 10, '4': 1, '5': 4, '10': 'feeSubsAmount'},
    const {'1': 'signup_rewards_count', '3': 11, '4': 1, '5': 4, '10': 'signupRewardsCount'},
    const {'1': 'signup_rewards_amount', '3': 12, '4': 1, '5': 4, '10': 'signupRewardsAmount'},
    const {'1': 'referral_rewards_count', '3': 13, '4': 1, '5': 4, '10': 'referralRewardsCount'},
    const {'1': 'referral_rewards_amount', '3': 14, '4': 1, '5': 4, '10': 'referralRewardsAmount'},
    const {'1': 'validator_rewards_count', '3': 15, '4': 1, '5': 4, '10': 'validatorRewardsCount'},
    const {'1': 'validator_rewards_amount', '3': 16, '4': 1, '5': 4, '10': 'validatorRewardsAmount'},
    const {'1': 'update_user_transactions_count', '3': 17, '4': 1, '5': 4, '10': 'updateUserTransactionsCount'},
    const {'1': 'exchange_rate', '3': 18, '4': 1, '5': 1, '10': 'exchangeRate'},
    const {'1': 'causes_rewards_amount', '3': 19, '4': 1, '5': 4, '10': 'causesRewardsAmount'},
  ],
};

/// Descriptor for `BlockchainStats`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List blockchainStatsDescriptor = $convert.base64Decode('Cg9CbG9ja2NoYWluU3RhdHMSJgoPbGFzdF9ibG9ja190aW1lGAEgASgEUg1sYXN0QmxvY2tUaW1lEh0KCnRpcF9oZWlnaHQYAiABKARSCXRpcEhlaWdodBItChJ0cmFuc2FjdGlvbnNfY291bnQYAyABKARSEXRyYW5zYWN0aW9uc0NvdW50Ej4KG3BheW1lbnRzX3RyYW5zYWN0aW9uc19jb3VudBgEIAEoBFIZcGF5bWVudHNUcmFuc2FjdGlvbnNDb3VudBIfCgt1c2Vyc19jb3VudBgFIAEoBFIKdXNlcnNDb3VudBIfCgtmZWVzX2Ftb3VudBgGIAEoBFIKZmVlc0Ftb3VudBIjCg1taW50ZWRfYW1vdW50GAcgASgEUgxtaW50ZWRBbW91bnQSIAoLY2lyY3VsYXRpb24YCCABKARSC2NpcmN1bGF0aW9uEiQKDmZlZV9zdWJzX2NvdW50GAkgASgEUgxmZWVTdWJzQ291bnQSJgoPZmVlX3N1YnNfYW1vdW50GAogASgEUg1mZWVTdWJzQW1vdW50EjAKFHNpZ251cF9yZXdhcmRzX2NvdW50GAsgASgEUhJzaWdudXBSZXdhcmRzQ291bnQSMgoVc2lnbnVwX3Jld2FyZHNfYW1vdW50GAwgASgEUhNzaWdudXBSZXdhcmRzQW1vdW50EjQKFnJlZmVycmFsX3Jld2FyZHNfY291bnQYDSABKARSFHJlZmVycmFsUmV3YXJkc0NvdW50EjYKF3JlZmVycmFsX3Jld2FyZHNfYW1vdW50GA4gASgEUhVyZWZlcnJhbFJld2FyZHNBbW91bnQSNgoXdmFsaWRhdG9yX3Jld2FyZHNfY291bnQYDyABKARSFXZhbGlkYXRvclJld2FyZHNDb3VudBI4Chh2YWxpZGF0b3JfcmV3YXJkc19hbW91bnQYECABKARSFnZhbGlkYXRvclJld2FyZHNBbW91bnQSQwoedXBkYXRlX3VzZXJfdHJhbnNhY3Rpb25zX2NvdW50GBEgASgEUht1cGRhdGVVc2VyVHJhbnNhY3Rpb25zQ291bnQSIwoNZXhjaGFuZ2VfcmF0ZRgSIAEoAVIMZXhjaGFuZ2VSYXRlEjIKFWNhdXNlc19yZXdhcmRzX2Ftb3VudBgTIAEoBFITY2F1c2VzUmV3YXJkc0Ftb3VudA==');
@$core.Deprecated('Use blockEventDescriptor instead')
const BlockEvent$json = const {
  '1': 'BlockEvent',
  '2': const [
    const {'1': 'timestamp', '3': 1, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'height', '3': 2, '4': 1, '5': 4, '10': 'height'},
    const {'1': 'block_hash', '3': 3, '4': 1, '5': 12, '10': 'blockHash'},
    const {'1': 'transactions_events', '3': 4, '4': 3, '5': 11, '6': '.karma_coin.core_types.TransactionEvent', '10': 'transactionsEvents'},
    const {'1': 'signups_count', '3': 5, '4': 1, '5': 4, '10': 'signupsCount'},
    const {'1': 'payments_count', '3': 6, '4': 1, '5': 4, '10': 'paymentsCount'},
    const {'1': 'user_updates_count', '3': 7, '4': 1, '5': 4, '10': 'userUpdatesCount'},
    const {'1': 'fees_amount', '3': 8, '4': 1, '5': 4, '10': 'feesAmount'},
    const {'1': 'signup_rewards_amount', '3': 9, '4': 1, '5': 4, '10': 'signupRewardsAmount'},
    const {'1': 'referral_rewards_amount', '3': 10, '4': 1, '5': 4, '10': 'referralRewardsAmount'},
    const {'1': 'referral_rewards_count', '3': 11, '4': 1, '5': 4, '10': 'referralRewardsCount'},
    const {'1': 'reward', '3': 12, '4': 1, '5': 4, '10': 'reward'},
  ],
};

/// Descriptor for `BlockEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List blockEventDescriptor = $convert.base64Decode('CgpCbG9ja0V2ZW50EhwKCXRpbWVzdGFtcBgBIAEoBFIJdGltZXN0YW1wEhYKBmhlaWdodBgCIAEoBFIGaGVpZ2h0Eh0KCmJsb2NrX2hhc2gYAyABKAxSCWJsb2NrSGFzaBJYChN0cmFuc2FjdGlvbnNfZXZlbnRzGAQgAygLMicua2FybWFfY29pbi5jb3JlX3R5cGVzLlRyYW5zYWN0aW9uRXZlbnRSEnRyYW5zYWN0aW9uc0V2ZW50cxIjCg1zaWdudXBzX2NvdW50GAUgASgEUgxzaWdudXBzQ291bnQSJQoOcGF5bWVudHNfY291bnQYBiABKARSDXBheW1lbnRzQ291bnQSLAoSdXNlcl91cGRhdGVzX2NvdW50GAcgASgEUhB1c2VyVXBkYXRlc0NvdW50Eh8KC2ZlZXNfYW1vdW50GAggASgEUgpmZWVzQW1vdW50EjIKFXNpZ251cF9yZXdhcmRzX2Ftb3VudBgJIAEoBFITc2lnbnVwUmV3YXJkc0Ftb3VudBI2ChdyZWZlcnJhbF9yZXdhcmRzX2Ftb3VudBgKIAEoBFIVcmVmZXJyYWxSZXdhcmRzQW1vdW50EjQKFnJlZmVycmFsX3Jld2FyZHNfY291bnQYCyABKARSFHJlZmVycmFsUmV3YXJkc0NvdW50EhYKBnJld2FyZBgMIAEoBFIGcmV3YXJk');
