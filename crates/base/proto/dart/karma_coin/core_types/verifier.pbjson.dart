///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/verifier.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use sendVerificationCodeRequestDescriptor instead')
const SendVerificationCodeRequest$json = const {
  '1': 'SendVerificationCodeRequest',
  '2': const [
    const {'1': 'mobile_number', '3': 1, '4': 1, '5': 9, '10': 'mobileNumber'},
  ],
};

/// Descriptor for `SendVerificationCodeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendVerificationCodeRequestDescriptor = $convert.base64Decode('ChtTZW5kVmVyaWZpY2F0aW9uQ29kZVJlcXVlc3QSIwoNbW9iaWxlX251bWJlchgBIAEoCVIMbW9iaWxlTnVtYmVy');
@$core.Deprecated('Use sendVerificationCodeResponseDescriptor instead')
const SendVerificationCodeResponse$json = const {
  '1': 'SendVerificationCodeResponse',
  '2': const [
    const {'1': 'session_id', '3': 1, '4': 1, '5': 9, '10': 'sessionId'},
  ],
};

/// Descriptor for `SendVerificationCodeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List sendVerificationCodeResponseDescriptor = $convert.base64Decode('ChxTZW5kVmVyaWZpY2F0aW9uQ29kZVJlc3BvbnNlEh0KCnNlc3Npb25faWQYASABKAlSCXNlc3Npb25JZA==');
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
@$core.Deprecated('Use verifyNumberResponseDescriptor instead')
const VerifyNumberResponse$json = const {
  '1': 'VerifyNumberResponse',
  '2': const [
    const {'1': 'user_verification_data', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.UserVerificationData', '10': 'userVerificationData'},
  ],
};

/// Descriptor for `VerifyNumberResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifyNumberResponseDescriptor = $convert.base64Decode('ChRWZXJpZnlOdW1iZXJSZXNwb25zZRJhChZ1c2VyX3ZlcmlmaWNhdGlvbl9kYXRhGAEgASgLMisua2FybWFfY29pbi5jb3JlX3R5cGVzLlVzZXJWZXJpZmljYXRpb25EYXRhUhR1c2VyVmVyaWZpY2F0aW9uRGF0YQ==');
@$core.Deprecated('Use verifyNumberRequestDataExDescriptor instead')
const VerifyNumberRequestDataEx$json = const {
  '1': 'VerifyNumberRequestDataEx',
  '2': const [
    const {'1': 'timestamp', '3': 1, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'account_id', '3': 2, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
    const {'1': 'mobile_number', '3': 3, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'mobileNumber'},
    const {'1': 'requested_user_name', '3': 4, '4': 1, '5': 9, '10': 'requestedUserName'},
    const {'1': 'bypass_token', '3': 5, '4': 1, '5': 12, '10': 'bypassToken'},
    const {'1': 'verification_code', '3': 6, '4': 1, '5': 9, '10': 'verificationCode'},
    const {'1': 'verification_sid', '3': 7, '4': 1, '5': 9, '10': 'verificationSid'},
  ],
};

/// Descriptor for `VerifyNumberRequestDataEx`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifyNumberRequestDataExDescriptor = $convert.base64Decode('ChlWZXJpZnlOdW1iZXJSZXF1ZXN0RGF0YUV4EhwKCXRpbWVzdGFtcBgBIAEoBFIJdGltZXN0YW1wEj8KCmFjY291bnRfaWQYAiABKAsyIC5rYXJtYV9jb2luLmNvcmVfdHlwZXMuQWNjb3VudElkUglhY2NvdW50SWQSSAoNbW9iaWxlX251bWJlchgDIAEoCzIjLmthcm1hX2NvaW4uY29yZV90eXBlcy5Nb2JpbGVOdW1iZXJSDG1vYmlsZU51bWJlchIuChNyZXF1ZXN0ZWRfdXNlcl9uYW1lGAQgASgJUhFyZXF1ZXN0ZWRVc2VyTmFtZRIhCgxieXBhc3NfdG9rZW4YBSABKAxSC2J5cGFzc1Rva2VuEisKEXZlcmlmaWNhdGlvbl9jb2RlGAYgASgJUhB2ZXJpZmljYXRpb25Db2RlEikKEHZlcmlmaWNhdGlvbl9zaWQYByABKAlSD3ZlcmlmaWNhdGlvblNpZA==');
@$core.Deprecated('Use verifyNumberRequestExDescriptor instead')
const VerifyNumberRequestEx$json = const {
  '1': 'VerifyNumberRequestEx',
  '2': const [
    const {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
    const {'1': 'signature', '3': 2, '4': 1, '5': 12, '10': 'signature'},
  ],
};

/// Descriptor for `VerifyNumberRequestEx`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List verifyNumberRequestExDescriptor = $convert.base64Decode('ChVWZXJpZnlOdW1iZXJSZXF1ZXN0RXgSEgoEZGF0YRgBIAEoDFIEZGF0YRIcCglzaWduYXR1cmUYAiABKAxSCXNpZ25hdHVyZQ==');
@$core.Deprecated('Use smsInviteMetadataDescriptor instead')
const SmsInviteMetadata$json = const {
  '1': 'SmsInviteMetadata',
  '2': const [
    const {'1': 'mobile_number', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.MobileNumber', '10': 'mobileNumber'},
    const {'1': 'last_message_sent_time_stamp', '3': 2, '4': 1, '5': 4, '10': 'lastMessageSentTimeStamp'},
    const {'1': 'messages_sent', '3': 3, '4': 1, '5': 13, '10': 'messagesSent'},
    const {'1': 'inviter_account_id', '3': 4, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'inviterAccountId'},
    const {'1': 'invite_tx_hash', '3': 5, '4': 1, '5': 12, '10': 'inviteTxHash'},
  ],
};

/// Descriptor for `SmsInviteMetadata`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List smsInviteMetadataDescriptor = $convert.base64Decode('ChFTbXNJbnZpdGVNZXRhZGF0YRJICg1tb2JpbGVfbnVtYmVyGAEgASgLMiMua2FybWFfY29pbi5jb3JlX3R5cGVzLk1vYmlsZU51bWJlclIMbW9iaWxlTnVtYmVyEj4KHGxhc3RfbWVzc2FnZV9zZW50X3RpbWVfc3RhbXAYAiABKARSGGxhc3RNZXNzYWdlU2VudFRpbWVTdGFtcBIjCg1tZXNzYWdlc19zZW50GAMgASgNUgxtZXNzYWdlc1NlbnQSTgoSaW52aXRlcl9hY2NvdW50X2lkGAQgASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLkFjY291bnRJZFIQaW52aXRlckFjY291bnRJZBIkCg5pbnZpdGVfdHhfaGFzaBgFIAEoDFIMaW52aXRlVHhIYXNo');
