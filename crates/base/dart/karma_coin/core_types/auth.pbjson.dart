///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/auth.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use authResultDescriptor instead')
const AuthResult$json = const {
  '1': 'AuthResult',
  '2': const [
    const {'1': 'AUTH_RESULT_USER_AUTHENTICATED', '2': 0},
    const {'1': 'AUTH_RESULT_USER_NOT_FOUND', '2': 1},
    const {'1': 'AUTH_RESULT_ACCOUNT_ID_MISMATCH', '2': 2},
  ],
};

/// Descriptor for `AuthResult`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List authResultDescriptor = $convert.base64Decode('CgpBdXRoUmVzdWx0EiIKHkFVVEhfUkVTVUxUX1VTRVJfQVVUSEVOVElDQVRFRBAAEh4KGkFVVEhfUkVTVUxUX1VTRVJfTk9UX0ZPVU5EEAESIwofQVVUSF9SRVNVTFRfQUNDT1VOVF9JRF9NSVNNQVRDSBAC');
@$core.Deprecated('Use authRequestDescriptor instead')
const AuthRequest$json = const {
  '1': 'AuthRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 11, '6': '.karma_coin.core_types.AccountId', '10': 'accountId'},
    const {'1': 'phone_number', '3': 2, '4': 1, '5': 9, '10': 'phoneNumber'},
  ],
};

/// Descriptor for `AuthRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authRequestDescriptor = $convert.base64Decode('CgtBdXRoUmVxdWVzdBI/CgphY2NvdW50X2lkGAEgASgLMiAua2FybWFfY29pbi5jb3JlX3R5cGVzLkFjY291bnRJZFIJYWNjb3VudElkEiEKDHBob25lX251bWJlchgCIAEoCVILcGhvbmVOdW1iZXI=');
@$core.Deprecated('Use authResponseDescriptor instead')
const AuthResponse$json = const {
  '1': 'AuthResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 14, '6': '.karma_coin.auth.AuthResult', '10': 'result'},
  ],
};

/// Descriptor for `AuthResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List authResponseDescriptor = $convert.base64Decode('CgxBdXRoUmVzcG9uc2USMwoGcmVzdWx0GAEgASgOMhsua2FybWFfY29pbi5hdXRoLkF1dGhSZXN1bHRSBnJlc3VsdA==');
