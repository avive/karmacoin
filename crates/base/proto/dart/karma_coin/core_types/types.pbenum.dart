///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/types.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class KeyScheme extends $pb.ProtobufEnum {
  static const KeyScheme KEY_SCHEME_ED25519 = KeyScheme._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'KEY_SCHEME_ED25519');

  static const $core.List<KeyScheme> values = <KeyScheme> [
    KEY_SCHEME_ED25519,
  ];

  static final $core.Map<$core.int, KeyScheme> _byValue = $pb.ProtobufEnum.initByValue(values);
  static KeyScheme? valueOf($core.int value) => _byValue[value];

  const KeyScheme._($core.int v, $core.String n) : super(v, n);
}

class TransactionType extends $pb.ProtobufEnum {
  static const TransactionType TRANSACTION_TYPE_PAYMENT_V1 = TransactionType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_TYPE_PAYMENT_V1');
  static const TransactionType TRANSACTION_TYPE_NEW_USER_V1 = TransactionType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_TYPE_NEW_USER_V1');
  static const TransactionType TRANSACTION_TYPE_UPDATE_USER_V1 = TransactionType._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_TYPE_UPDATE_USER_V1');
  static const TransactionType TRANSACTION_TYPE_DELETE_USER_V1 = TransactionType._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_TYPE_DELETE_USER_V1');

  static const $core.List<TransactionType> values = <TransactionType> [
    TRANSACTION_TYPE_PAYMENT_V1,
    TRANSACTION_TYPE_NEW_USER_V1,
    TRANSACTION_TYPE_UPDATE_USER_V1,
    TRANSACTION_TYPE_DELETE_USER_V1,
  ];

  static final $core.Map<$core.int, TransactionType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static TransactionType? valueOf($core.int value) => _byValue[value];

  const TransactionType._($core.int v, $core.String n) : super(v, n);
}

class VerificationResult extends $pb.ProtobufEnum {
  static const VerificationResult VERIFICATION_RESULT_UNSPECIFIED = VerificationResult._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_RESULT_UNSPECIFIED');
  static const VerificationResult VERIFICATION_RESULT_USER_NAME_TAKEN = VerificationResult._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_RESULT_USER_NAME_TAKEN');
  static const VerificationResult VERIFICATION_RESULT_VERIFIED = VerificationResult._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_RESULT_VERIFIED');
  static const VerificationResult VERIFICATION_RESULT_UNVERIFIED = VerificationResult._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_RESULT_UNVERIFIED');
  static const VerificationResult VERIFICATION_RESULT_MISSING_DATA = VerificationResult._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_RESULT_MISSING_DATA');
  static const VerificationResult VERIFICATION_RESULT_INVALID_SIGNATURE = VerificationResult._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_RESULT_INVALID_SIGNATURE');
  static const VerificationResult VERIFICATION_RESULT_ACCOUNT_MISMATCH = VerificationResult._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_RESULT_ACCOUNT_MISMATCH');

  static const $core.List<VerificationResult> values = <VerificationResult> [
    VERIFICATION_RESULT_UNSPECIFIED,
    VERIFICATION_RESULT_USER_NAME_TAKEN,
    VERIFICATION_RESULT_VERIFIED,
    VERIFICATION_RESULT_UNVERIFIED,
    VERIFICATION_RESULT_MISSING_DATA,
    VERIFICATION_RESULT_INVALID_SIGNATURE,
    VERIFICATION_RESULT_ACCOUNT_MISMATCH,
  ];

  static final $core.Map<$core.int, VerificationResult> _byValue = $pb.ProtobufEnum.initByValue(values);
  static VerificationResult? valueOf($core.int value) => _byValue[value];

  const VerificationResult._($core.int v, $core.String n) : super(v, n);
}

class TransactionStatus extends $pb.ProtobufEnum {
  static const TransactionStatus TRANSACTION_STATUS_UNKNOWN = TransactionStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_STATUS_UNKNOWN');
  static const TransactionStatus TRANSACTION_STATUS_NOT_SUBMITTED = TransactionStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_STATUS_NOT_SUBMITTED');
  static const TransactionStatus TRANSACTION_STATUS_SUBMITTED = TransactionStatus._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_STATUS_SUBMITTED');
  static const TransactionStatus TRANSACTION_STATUS_REJECTED = TransactionStatus._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_STATUS_REJECTED');
  static const TransactionStatus TRANSACTION_STATUS_ON_CHAIN = TransactionStatus._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TRANSACTION_STATUS_ON_CHAIN');

  static const $core.List<TransactionStatus> values = <TransactionStatus> [
    TRANSACTION_STATUS_UNKNOWN,
    TRANSACTION_STATUS_NOT_SUBMITTED,
    TRANSACTION_STATUS_SUBMITTED,
    TRANSACTION_STATUS_REJECTED,
    TRANSACTION_STATUS_ON_CHAIN,
  ];

  static final $core.Map<$core.int, TransactionStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static TransactionStatus? valueOf($core.int value) => _byValue[value];

  const TransactionStatus._($core.int v, $core.String n) : super(v, n);
}

class FeeType extends $pb.ProtobufEnum {
  static const FeeType FEE_TYPE_MINT = FeeType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FEE_TYPE_MINT');
  static const FeeType FEE_TYPE_USER = FeeType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FEE_TYPE_USER');

  static const $core.List<FeeType> values = <FeeType> [
    FEE_TYPE_MINT,
    FEE_TYPE_USER,
  ];

  static final $core.Map<$core.int, FeeType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static FeeType? valueOf($core.int value) => _byValue[value];

  const FeeType._($core.int v, $core.String n) : super(v, n);
}

class ExecutionResult extends $pb.ProtobufEnum {
  static const ExecutionResult EXECUTION_RESULT_EXECUTED = ExecutionResult._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_RESULT_EXECUTED');
  static const ExecutionResult EXECUTION_RESULT_INVALID = ExecutionResult._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_RESULT_INVALID');

  static const $core.List<ExecutionResult> values = <ExecutionResult> [
    EXECUTION_RESULT_EXECUTED,
    EXECUTION_RESULT_INVALID,
  ];

  static final $core.Map<$core.int, ExecutionResult> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ExecutionResult? valueOf($core.int value) => _byValue[value];

  const ExecutionResult._($core.int v, $core.String n) : super(v, n);
}

class ExecutionInfo extends $pb.ProtobufEnum {
  static const ExecutionInfo EXECUTION_INFO_UNKNOWN = ExecutionInfo._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_UNKNOWN');
  static const ExecutionInfo EXECUTION_INFO_NICKNAME_UPDATED = ExecutionInfo._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_NICKNAME_UPDATED');
  static const ExecutionInfo EXECUTION_INFO_NICKNAME_NOT_AVAILABLE = ExecutionInfo._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_NICKNAME_NOT_AVAILABLE');
  static const ExecutionInfo EXECUTION_INFO_NICKNAME_INVALID = ExecutionInfo._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_NICKNAME_INVALID');
  static const ExecutionInfo EXECUTION_INFO_NUMBER_UPDATED = ExecutionInfo._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_NUMBER_UPDATED');
  static const ExecutionInfo EXECUTION_INFO_ACCOUNT_CREATED = ExecutionInfo._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_ACCOUNT_CREATED');
  static const ExecutionInfo EXECUTION_INFO_PAYMENT_CONFIRMED = ExecutionInfo._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_PAYMENT_CONFIRMED');
  static const ExecutionInfo EXECUTION_INFO_INVALID_DATA = ExecutionInfo._(7, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_INVALID_DATA');
  static const ExecutionInfo EXECUTION_INFO_ACCOUNT_ALREADY_EXISTS = ExecutionInfo._(8, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_ACCOUNT_ALREADY_EXISTS');
  static const ExecutionInfo EXECUTION_INFO_TX_FEE_TOO_LOW = ExecutionInfo._(9, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_TX_FEE_TOO_LOW');
  static const ExecutionInfo EXECUTION_INFO_INTERNAL_NODE_ERROR = ExecutionInfo._(10, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXECUTION_INFO_INTERNAL_NODE_ERROR');

  static const $core.List<ExecutionInfo> values = <ExecutionInfo> [
    EXECUTION_INFO_UNKNOWN,
    EXECUTION_INFO_NICKNAME_UPDATED,
    EXECUTION_INFO_NICKNAME_NOT_AVAILABLE,
    EXECUTION_INFO_NICKNAME_INVALID,
    EXECUTION_INFO_NUMBER_UPDATED,
    EXECUTION_INFO_ACCOUNT_CREATED,
    EXECUTION_INFO_PAYMENT_CONFIRMED,
    EXECUTION_INFO_INVALID_DATA,
    EXECUTION_INFO_ACCOUNT_ALREADY_EXISTS,
    EXECUTION_INFO_TX_FEE_TOO_LOW,
    EXECUTION_INFO_INTERNAL_NODE_ERROR,
  ];

  static final $core.Map<$core.int, ExecutionInfo> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ExecutionInfo? valueOf($core.int value) => _byValue[value];

  const ExecutionInfo._($core.int v, $core.String n) : super(v, n);
}

