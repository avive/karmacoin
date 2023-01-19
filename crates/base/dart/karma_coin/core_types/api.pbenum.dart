///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class SubmitTransactionResult extends $pb.ProtobufEnum {
  static const SubmitTransactionResult SUBMIT_TRANSACTION_RESULT_REJECTED = SubmitTransactionResult._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'SUBMIT_TRANSACTION_RESULT_REJECTED');
  static const SubmitTransactionResult SUBMIT_TRANSACTION_RESULT_SUBMITTED = SubmitTransactionResult._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'SUBMIT_TRANSACTION_RESULT_SUBMITTED');

  static const $core.List<SubmitTransactionResult> values = <SubmitTransactionResult> [
    SUBMIT_TRANSACTION_RESULT_REJECTED,
    SUBMIT_TRANSACTION_RESULT_SUBMITTED,
  ];

  static final $core.Map<$core.int, SubmitTransactionResult> _byValue = $pb.ProtobufEnum.initByValue(values);
  static SubmitTransactionResult? valueOf($core.int value) => _byValue[value];

  const SubmitTransactionResult._($core.int v, $core.String n) : super(v, n);
}

