///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/auth.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class AuthResult extends $pb.ProtobufEnum {
  static const AuthResult AUTH_RESULT_USER_AUTHENTICATED = AuthResult._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'AUTH_RESULT_USER_AUTHENTICATED');
  static const AuthResult AUTH_RESULT_USER_NOT_FOUND = AuthResult._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'AUTH_RESULT_USER_NOT_FOUND');
  static const AuthResult AUTH_RESULT_ACCOUNT_ID_MISMATCH = AuthResult._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'AUTH_RESULT_ACCOUNT_ID_MISMATCH');

  static const $core.List<AuthResult> values = <AuthResult> [
    AUTH_RESULT_USER_AUTHENTICATED,
    AUTH_RESULT_USER_NOT_FOUND,
    AUTH_RESULT_ACCOUNT_ID_MISMATCH,
  ];

  static final $core.Map<$core.int, AuthResult> _byValue = $pb.ProtobufEnum.initByValue(values);
  static AuthResult? valueOf($core.int value) => _byValue[value];

  const AuthResult._($core.int v, $core.String n) : super(v, n);
}

