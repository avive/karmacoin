///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/verifier.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use verifierInfoDescriptor instead')
const VerifierInfo$json = const {
  '1': 'VerifierInfo',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'account_id', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
    const {'1': 'verifier_endpoint_ip4', '3': 3, '4': 1, '5': 9, '10': 'verifierEndpointIp4'},
    const {'1': 'verifier_endpoint_ip6', '3': 4, '4': 1, '5': 9, '10': 'verifierEndpointIp6'},
    const {'1': 'api_endpoint_ip4', '3': 5, '4': 1, '5': 9, '10': 'apiEndpointIp4'},
    const {'1': 'api_endpoint_ip6', '3': 6, '4': 1, '5': 9, '10': 'apiEndpointIp6'},
    const {'1': 'signature', '3': 7, '4': 1, '5': 11, '6': '.karma_coin.core_types.Signature', '10': 'signature'},
  ],
};

/// Descriptor for `VerifierInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifierInfoDescriptor = $convert.base64Decode('CgxWZXJpZmllckluZm8SEgoEbmFtZRgBIAEoCVIEbmFtZRI/CgphY2NvdW50X2lkGAIgASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLkFjY291bnRJZFIJYWNjb3VudElkEjIKFXZlcmlmaWVyX2VuZHBvaW50X2lwNBgDIAEoCVITdmVyaWZpZXJFbmRwb2ludElwNBIyChV2ZXJpZmllcl9lbmRwb2ludF9pcDYYBCABKAlSE3ZlcmlmaWVyRW5kcG9pbnRJcDYSKAoQYXBpX2VuZHBvaW50X2lwNBgFIAEoCVIOYXBpRW5kcG9pbnRJcDQSKAoQYXBpX2VuZHBvaW50X2lwNhgGIAEoCVIOYXBpRW5kcG9pbnRJcDYSPgoJc2lnbmF0dXJlGAcgASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLlNpZ25hdHVyZVIJc2lnbmF0dXJl');
@$core.Deprecated('Use verifyNumberRequestDescriptor instead')
const VerifyNumberRequest$json = const {
  '1': 'VerifyNumberRequest',
  '2': const [
    const {'1': 'timestamp', '3': 1, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'account_id', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
    const {'1': 'mobile_number', '3': 3, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'mobileNumber'},
    const {'1': 'requested_user_name', '3': 4, '4': 1, '5': 9, '10': 'requestedUserName'},
    const {'1': 'signature', '3': 5, '4': 1, '5': 11, '6': '.karma_coin.core_types.Signature', '10': 'signature'},
  ],
};

/// Descriptor for `VerifyNumberRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifyNumberRequestDescriptor = $convert.base64Decode('ChNWZXJpZnlOdW1iZXJSZXF1ZXN0EhwKCXRpbWVzdGFtcBgBIAEoBFIJdGltZXN0YW1wEj8KCmFjY291bnRfaWQYAiABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQWNjb3VudElkUglhY2NvdW50SWQSSAoNbW9iaWxlX251bWJlchgDIAEoCzIjLmthcm1hX2NvaW4uY29yZV90eXBlcy5Nb2JpbGVOdW1iZXJSDG1vYmlsZU51bWJlchIuChNyZXF1ZXN0ZWRfdXNlcl9uYW1lGAQgASgJUhFyZXF1ZXN0ZWRVc2VyTmFtZRI+CglzaWduYXR1cmUYBSABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuU2lnbmF0dXJlUglzaWduYXR1cmU=');
