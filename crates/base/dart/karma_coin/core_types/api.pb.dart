///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'types.pb.dart' as $2;

import 'api.pbenum.dart';

export 'api.pbenum.dart';

class GetUserInfoByUserNameRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByUserNameRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userName')
    ..hasRequiredFields = false
  ;

  GetUserInfoByUserNameRequest._() : super();
  factory GetUserInfoByUserNameRequest({
    $core.String? userName,
  }) {
    final _result = create();
    if (userName != null) {
      _result.userName = userName;
    }
    return _result;
  }
  factory GetUserInfoByUserNameRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetUserInfoByUserNameRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetUserInfoByUserNameRequest clone() => GetUserInfoByUserNameRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetUserInfoByUserNameRequest copyWith(void Function(GetUserInfoByUserNameRequest) updates) => super.copyWith((message) => updates(message as GetUserInfoByUserNameRequest)) as GetUserInfoByUserNameRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByUserNameRequest create() => GetUserInfoByUserNameRequest._();
  GetUserInfoByUserNameRequest createEmptyInstance() => create();
  static $pb.PbList<GetUserInfoByUserNameRequest> createRepeated() => $pb.PbList<GetUserInfoByUserNameRequest>();
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByUserNameRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetUserInfoByUserNameRequest>(create);
  static GetUserInfoByUserNameRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get userName => $_getSZ(0);
  @$pb.TagNumber(1)
  set userName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserName() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserName() => clearField(1);
}

class GetUserInfoByUserNameResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByUserNameResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.User>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', subBuilder: $2.User.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByUserNameResponse._() : super();
  factory GetUserInfoByUserNameResponse({
    $2.User? user,
  }) {
    final _result = create();
    if (user != null) {
      _result.user = user;
    }
    return _result;
  }
  factory GetUserInfoByUserNameResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetUserInfoByUserNameResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetUserInfoByUserNameResponse clone() => GetUserInfoByUserNameResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetUserInfoByUserNameResponse copyWith(void Function(GetUserInfoByUserNameResponse) updates) => super.copyWith((message) => updates(message as GetUserInfoByUserNameResponse)) as GetUserInfoByUserNameResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByUserNameResponse create() => GetUserInfoByUserNameResponse._();
  GetUserInfoByUserNameResponse createEmptyInstance() => create();
  static $pb.PbList<GetUserInfoByUserNameResponse> createRepeated() => $pb.PbList<GetUserInfoByUserNameResponse>();
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByUserNameResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetUserInfoByUserNameResponse>(create);
  static GetUserInfoByUserNameResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $2.User get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($2.User v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);
  @$pb.TagNumber(1)
  $2.User ensureUser() => $_ensure(0);
}

class SubmitTransactionRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SubmitTransactionRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.SignedTransaction>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transaction', subBuilder: $2.SignedTransaction.create)
    ..hasRequiredFields = false
  ;

  SubmitTransactionRequest._() : super();
  factory SubmitTransactionRequest({
    $2.SignedTransaction? transaction,
  }) {
    final _result = create();
    if (transaction != null) {
      _result.transaction = transaction;
    }
    return _result;
  }
  factory SubmitTransactionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SubmitTransactionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SubmitTransactionRequest clone() => SubmitTransactionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SubmitTransactionRequest copyWith(void Function(SubmitTransactionRequest) updates) => super.copyWith((message) => updates(message as SubmitTransactionRequest)) as SubmitTransactionRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SubmitTransactionRequest create() => SubmitTransactionRequest._();
  SubmitTransactionRequest createEmptyInstance() => create();
  static $pb.PbList<SubmitTransactionRequest> createRepeated() => $pb.PbList<SubmitTransactionRequest>();
  @$core.pragma('dart2js:noInline')
  static SubmitTransactionRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SubmitTransactionRequest>(create);
  static SubmitTransactionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $2.SignedTransaction get transaction => $_getN(0);
  @$pb.TagNumber(1)
  set transaction($2.SignedTransaction v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransaction() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransaction() => clearField(1);
  @$pb.TagNumber(1)
  $2.SignedTransaction ensureTransaction() => $_ensure(0);
}

class SubmitTransactionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SubmitTransactionResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..e<SubmitTransactionResult>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'submitTransactionResult', $pb.PbFieldType.OE, defaultOrMaker: SubmitTransactionResult.SUBMIT_TRANSACTION_RESULT_REJECTED, valueOf: SubmitTransactionResult.valueOf, enumValues: SubmitTransactionResult.values)
    ..hasRequiredFields = false
  ;

  SubmitTransactionResponse._() : super();
  factory SubmitTransactionResponse({
    SubmitTransactionResult? submitTransactionResult,
  }) {
    final _result = create();
    if (submitTransactionResult != null) {
      _result.submitTransactionResult = submitTransactionResult;
    }
    return _result;
  }
  factory SubmitTransactionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SubmitTransactionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SubmitTransactionResponse clone() => SubmitTransactionResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SubmitTransactionResponse copyWith(void Function(SubmitTransactionResponse) updates) => super.copyWith((message) => updates(message as SubmitTransactionResponse)) as SubmitTransactionResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SubmitTransactionResponse create() => SubmitTransactionResponse._();
  SubmitTransactionResponse createEmptyInstance() => create();
  static $pb.PbList<SubmitTransactionResponse> createRepeated() => $pb.PbList<SubmitTransactionResponse>();
  @$core.pragma('dart2js:noInline')
  static SubmitTransactionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SubmitTransactionResponse>(create);
  static SubmitTransactionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  SubmitTransactionResult get submitTransactionResult => $_getN(0);
  @$pb.TagNumber(1)
  set submitTransactionResult(SubmitTransactionResult v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasSubmitTransactionResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearSubmitTransactionResult() => clearField(1);
}

class GetUserInfoByNumberRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByNumberRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.MobileNumber>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: $2.MobileNumber.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByNumberRequest._() : super();
  factory GetUserInfoByNumberRequest({
    $2.MobileNumber? mobileNumber,
  }) {
    final _result = create();
    if (mobileNumber != null) {
      _result.mobileNumber = mobileNumber;
    }
    return _result;
  }
  factory GetUserInfoByNumberRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetUserInfoByNumberRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetUserInfoByNumberRequest clone() => GetUserInfoByNumberRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetUserInfoByNumberRequest copyWith(void Function(GetUserInfoByNumberRequest) updates) => super.copyWith((message) => updates(message as GetUserInfoByNumberRequest)) as GetUserInfoByNumberRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByNumberRequest create() => GetUserInfoByNumberRequest._();
  GetUserInfoByNumberRequest createEmptyInstance() => create();
  static $pb.PbList<GetUserInfoByNumberRequest> createRepeated() => $pb.PbList<GetUserInfoByNumberRequest>();
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByNumberRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetUserInfoByNumberRequest>(create);
  static GetUserInfoByNumberRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $2.MobileNumber get mobileNumber => $_getN(0);
  @$pb.TagNumber(1)
  set mobileNumber($2.MobileNumber v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasMobileNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearMobileNumber() => clearField(1);
  @$pb.TagNumber(1)
  $2.MobileNumber ensureMobileNumber() => $_ensure(0);
}

class GetUserInfoByNumberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByNumberResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.User>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', subBuilder: $2.User.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByNumberResponse._() : super();
  factory GetUserInfoByNumberResponse({
    $2.User? user,
  }) {
    final _result = create();
    if (user != null) {
      _result.user = user;
    }
    return _result;
  }
  factory GetUserInfoByNumberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetUserInfoByNumberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetUserInfoByNumberResponse clone() => GetUserInfoByNumberResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetUserInfoByNumberResponse copyWith(void Function(GetUserInfoByNumberResponse) updates) => super.copyWith((message) => updates(message as GetUserInfoByNumberResponse)) as GetUserInfoByNumberResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByNumberResponse create() => GetUserInfoByNumberResponse._();
  GetUserInfoByNumberResponse createEmptyInstance() => create();
  static $pb.PbList<GetUserInfoByNumberResponse> createRepeated() => $pb.PbList<GetUserInfoByNumberResponse>();
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByNumberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetUserInfoByNumberResponse>(create);
  static GetUserInfoByNumberResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $2.User get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($2.User v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);
  @$pb.TagNumber(1)
  $2.User ensureUser() => $_ensure(0);
}

class GetUserInfoByAccountRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByAccountRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: $2.AccountId.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByAccountRequest._() : super();
  factory GetUserInfoByAccountRequest({
    $2.AccountId? accountId,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    return _result;
  }
  factory GetUserInfoByAccountRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetUserInfoByAccountRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetUserInfoByAccountRequest clone() => GetUserInfoByAccountRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetUserInfoByAccountRequest copyWith(void Function(GetUserInfoByAccountRequest) updates) => super.copyWith((message) => updates(message as GetUserInfoByAccountRequest)) as GetUserInfoByAccountRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByAccountRequest create() => GetUserInfoByAccountRequest._();
  GetUserInfoByAccountRequest createEmptyInstance() => create();
  static $pb.PbList<GetUserInfoByAccountRequest> createRepeated() => $pb.PbList<GetUserInfoByAccountRequest>();
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByAccountRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetUserInfoByAccountRequest>(create);
  static GetUserInfoByAccountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $2.AccountId get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($2.AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);
  @$pb.TagNumber(1)
  $2.AccountId ensureAccountId() => $_ensure(0);
}

class GetUserInfoByAccountResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByAccountResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.User>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', subBuilder: $2.User.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByAccountResponse._() : super();
  factory GetUserInfoByAccountResponse({
    $2.User? user,
  }) {
    final _result = create();
    if (user != null) {
      _result.user = user;
    }
    return _result;
  }
  factory GetUserInfoByAccountResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetUserInfoByAccountResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetUserInfoByAccountResponse clone() => GetUserInfoByAccountResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetUserInfoByAccountResponse copyWith(void Function(GetUserInfoByAccountResponse) updates) => super.copyWith((message) => updates(message as GetUserInfoByAccountResponse)) as GetUserInfoByAccountResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByAccountResponse create() => GetUserInfoByAccountResponse._();
  GetUserInfoByAccountResponse createEmptyInstance() => create();
  static $pb.PbList<GetUserInfoByAccountResponse> createRepeated() => $pb.PbList<GetUserInfoByAccountResponse>();
  @$core.pragma('dart2js:noInline')
  static GetUserInfoByAccountResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetUserInfoByAccountResponse>(create);
  static GetUserInfoByAccountResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $2.User get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($2.User v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);
  @$pb.TagNumber(1)
  $2.User ensureUser() => $_ensure(0);
}

class GetGenesisDataRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetGenesisDataRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GetGenesisDataRequest._() : super();
  factory GetGenesisDataRequest() => create();
  factory GetGenesisDataRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetGenesisDataRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetGenesisDataRequest clone() => GetGenesisDataRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetGenesisDataRequest copyWith(void Function(GetGenesisDataRequest) updates) => super.copyWith((message) => updates(message as GetGenesisDataRequest)) as GetGenesisDataRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetGenesisDataRequest create() => GetGenesisDataRequest._();
  GetGenesisDataRequest createEmptyInstance() => create();
  static $pb.PbList<GetGenesisDataRequest> createRepeated() => $pb.PbList<GetGenesisDataRequest>();
  @$core.pragma('dart2js:noInline')
  static GetGenesisDataRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetGenesisDataRequest>(create);
  static GetGenesisDataRequest? _defaultInstance;
}

class GetGenesisDataResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetGenesisDataResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'netId', $pb.PbFieldType.OU3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'netName')
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'genesisTime', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardPhase1Alloc', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardPhase2Alloc', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardPhase1Amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardPhase2Amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardPhase3Start', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardPhase1Alloc', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardPhase2Alloc', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardPhase1Amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardPhase2Amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(13, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txFeeSubsidyMaxPerUser', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(14, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txFeeSubsidiesAlloc', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(15, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txFeeSubsidyMaxAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(16, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blockRewardAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(17, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blockRewardLastBlock', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(18, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'karmaRewardAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(19, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'karmaRewardAlloc', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(20, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'karmaRewardTopNUsers', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(21, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'treasuryPremintAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(22, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'treasuryAccountId')
    ..aOS(23, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'treasuryAccountName')
    ..pc<$2.CharTrait>(24, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'charTraits', $pb.PbFieldType.PM, subBuilder: $2.CharTrait.create)
    ..pc<$2.PhoneVerifier>(25, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verifiers', $pb.PbFieldType.PM, subBuilder: $2.PhoneVerifier.create)
    ..hasRequiredFields = false
  ;

  GetGenesisDataResponse._() : super();
  factory GetGenesisDataResponse({
    $core.int? netId,
    $core.String? netName,
    $fixnum.Int64? genesisTime,
    $fixnum.Int64? signupRewardPhase1Alloc,
    $fixnum.Int64? signupRewardPhase2Alloc,
    $fixnum.Int64? signupRewardPhase1Amount,
    $fixnum.Int64? signupRewardPhase2Amount,
    $fixnum.Int64? signupRewardPhase3Start,
    $fixnum.Int64? referralRewardPhase1Alloc,
    $fixnum.Int64? referralRewardPhase2Alloc,
    $fixnum.Int64? referralRewardPhase1Amount,
    $fixnum.Int64? referralRewardPhase2Amount,
    $fixnum.Int64? txFeeSubsidyMaxPerUser,
    $fixnum.Int64? txFeeSubsidiesAlloc,
    $fixnum.Int64? txFeeSubsidyMaxAmount,
    $fixnum.Int64? blockRewardAmount,
    $fixnum.Int64? blockRewardLastBlock,
    $fixnum.Int64? karmaRewardAmount,
    $fixnum.Int64? karmaRewardAlloc,
    $fixnum.Int64? karmaRewardTopNUsers,
    $fixnum.Int64? treasuryPremintAmount,
    $core.String? treasuryAccountId,
    $core.String? treasuryAccountName,
    $core.Iterable<$2.CharTrait>? charTraits,
    $core.Iterable<$2.PhoneVerifier>? verifiers,
  }) {
    final _result = create();
    if (netId != null) {
      _result.netId = netId;
    }
    if (netName != null) {
      _result.netName = netName;
    }
    if (genesisTime != null) {
      _result.genesisTime = genesisTime;
    }
    if (signupRewardPhase1Alloc != null) {
      _result.signupRewardPhase1Alloc = signupRewardPhase1Alloc;
    }
    if (signupRewardPhase2Alloc != null) {
      _result.signupRewardPhase2Alloc = signupRewardPhase2Alloc;
    }
    if (signupRewardPhase1Amount != null) {
      _result.signupRewardPhase1Amount = signupRewardPhase1Amount;
    }
    if (signupRewardPhase2Amount != null) {
      _result.signupRewardPhase2Amount = signupRewardPhase2Amount;
    }
    if (signupRewardPhase3Start != null) {
      _result.signupRewardPhase3Start = signupRewardPhase3Start;
    }
    if (referralRewardPhase1Alloc != null) {
      _result.referralRewardPhase1Alloc = referralRewardPhase1Alloc;
    }
    if (referralRewardPhase2Alloc != null) {
      _result.referralRewardPhase2Alloc = referralRewardPhase2Alloc;
    }
    if (referralRewardPhase1Amount != null) {
      _result.referralRewardPhase1Amount = referralRewardPhase1Amount;
    }
    if (referralRewardPhase2Amount != null) {
      _result.referralRewardPhase2Amount = referralRewardPhase2Amount;
    }
    if (txFeeSubsidyMaxPerUser != null) {
      _result.txFeeSubsidyMaxPerUser = txFeeSubsidyMaxPerUser;
    }
    if (txFeeSubsidiesAlloc != null) {
      _result.txFeeSubsidiesAlloc = txFeeSubsidiesAlloc;
    }
    if (txFeeSubsidyMaxAmount != null) {
      _result.txFeeSubsidyMaxAmount = txFeeSubsidyMaxAmount;
    }
    if (blockRewardAmount != null) {
      _result.blockRewardAmount = blockRewardAmount;
    }
    if (blockRewardLastBlock != null) {
      _result.blockRewardLastBlock = blockRewardLastBlock;
    }
    if (karmaRewardAmount != null) {
      _result.karmaRewardAmount = karmaRewardAmount;
    }
    if (karmaRewardAlloc != null) {
      _result.karmaRewardAlloc = karmaRewardAlloc;
    }
    if (karmaRewardTopNUsers != null) {
      _result.karmaRewardTopNUsers = karmaRewardTopNUsers;
    }
    if (treasuryPremintAmount != null) {
      _result.treasuryPremintAmount = treasuryPremintAmount;
    }
    if (treasuryAccountId != null) {
      _result.treasuryAccountId = treasuryAccountId;
    }
    if (treasuryAccountName != null) {
      _result.treasuryAccountName = treasuryAccountName;
    }
    if (charTraits != null) {
      _result.charTraits.addAll(charTraits);
    }
    if (verifiers != null) {
      _result.verifiers.addAll(verifiers);
    }
    return _result;
  }
  factory GetGenesisDataResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetGenesisDataResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetGenesisDataResponse clone() => GetGenesisDataResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetGenesisDataResponse copyWith(void Function(GetGenesisDataResponse) updates) => super.copyWith((message) => updates(message as GetGenesisDataResponse)) as GetGenesisDataResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetGenesisDataResponse create() => GetGenesisDataResponse._();
  GetGenesisDataResponse createEmptyInstance() => create();
  static $pb.PbList<GetGenesisDataResponse> createRepeated() => $pb.PbList<GetGenesisDataResponse>();
  @$core.pragma('dart2js:noInline')
  static GetGenesisDataResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetGenesisDataResponse>(create);
  static GetGenesisDataResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get netId => $_getIZ(0);
  @$pb.TagNumber(1)
  set netId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNetId() => $_has(0);
  @$pb.TagNumber(1)
  void clearNetId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get netName => $_getSZ(1);
  @$pb.TagNumber(2)
  set netName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNetName() => $_has(1);
  @$pb.TagNumber(2)
  void clearNetName() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get genesisTime => $_getI64(2);
  @$pb.TagNumber(3)
  set genesisTime($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasGenesisTime() => $_has(2);
  @$pb.TagNumber(3)
  void clearGenesisTime() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get signupRewardPhase1Alloc => $_getI64(3);
  @$pb.TagNumber(4)
  set signupRewardPhase1Alloc($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasSignupRewardPhase1Alloc() => $_has(3);
  @$pb.TagNumber(4)
  void clearSignupRewardPhase1Alloc() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get signupRewardPhase2Alloc => $_getI64(4);
  @$pb.TagNumber(5)
  set signupRewardPhase2Alloc($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasSignupRewardPhase2Alloc() => $_has(4);
  @$pb.TagNumber(5)
  void clearSignupRewardPhase2Alloc() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get signupRewardPhase1Amount => $_getI64(5);
  @$pb.TagNumber(6)
  set signupRewardPhase1Amount($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasSignupRewardPhase1Amount() => $_has(5);
  @$pb.TagNumber(6)
  void clearSignupRewardPhase1Amount() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get signupRewardPhase2Amount => $_getI64(6);
  @$pb.TagNumber(7)
  set signupRewardPhase2Amount($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasSignupRewardPhase2Amount() => $_has(6);
  @$pb.TagNumber(7)
  void clearSignupRewardPhase2Amount() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get signupRewardPhase3Start => $_getI64(7);
  @$pb.TagNumber(8)
  set signupRewardPhase3Start($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasSignupRewardPhase3Start() => $_has(7);
  @$pb.TagNumber(8)
  void clearSignupRewardPhase3Start() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get referralRewardPhase1Alloc => $_getI64(8);
  @$pb.TagNumber(9)
  set referralRewardPhase1Alloc($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasReferralRewardPhase1Alloc() => $_has(8);
  @$pb.TagNumber(9)
  void clearReferralRewardPhase1Alloc() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get referralRewardPhase2Alloc => $_getI64(9);
  @$pb.TagNumber(10)
  set referralRewardPhase2Alloc($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasReferralRewardPhase2Alloc() => $_has(9);
  @$pb.TagNumber(10)
  void clearReferralRewardPhase2Alloc() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get referralRewardPhase1Amount => $_getI64(10);
  @$pb.TagNumber(11)
  set referralRewardPhase1Amount($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasReferralRewardPhase1Amount() => $_has(10);
  @$pb.TagNumber(11)
  void clearReferralRewardPhase1Amount() => clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get referralRewardPhase2Amount => $_getI64(11);
  @$pb.TagNumber(12)
  set referralRewardPhase2Amount($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasReferralRewardPhase2Amount() => $_has(11);
  @$pb.TagNumber(12)
  void clearReferralRewardPhase2Amount() => clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get txFeeSubsidyMaxPerUser => $_getI64(12);
  @$pb.TagNumber(13)
  set txFeeSubsidyMaxPerUser($fixnum.Int64 v) { $_setInt64(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasTxFeeSubsidyMaxPerUser() => $_has(12);
  @$pb.TagNumber(13)
  void clearTxFeeSubsidyMaxPerUser() => clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get txFeeSubsidiesAlloc => $_getI64(13);
  @$pb.TagNumber(14)
  set txFeeSubsidiesAlloc($fixnum.Int64 v) { $_setInt64(13, v); }
  @$pb.TagNumber(14)
  $core.bool hasTxFeeSubsidiesAlloc() => $_has(13);
  @$pb.TagNumber(14)
  void clearTxFeeSubsidiesAlloc() => clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get txFeeSubsidyMaxAmount => $_getI64(14);
  @$pb.TagNumber(15)
  set txFeeSubsidyMaxAmount($fixnum.Int64 v) { $_setInt64(14, v); }
  @$pb.TagNumber(15)
  $core.bool hasTxFeeSubsidyMaxAmount() => $_has(14);
  @$pb.TagNumber(15)
  void clearTxFeeSubsidyMaxAmount() => clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get blockRewardAmount => $_getI64(15);
  @$pb.TagNumber(16)
  set blockRewardAmount($fixnum.Int64 v) { $_setInt64(15, v); }
  @$pb.TagNumber(16)
  $core.bool hasBlockRewardAmount() => $_has(15);
  @$pb.TagNumber(16)
  void clearBlockRewardAmount() => clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get blockRewardLastBlock => $_getI64(16);
  @$pb.TagNumber(17)
  set blockRewardLastBlock($fixnum.Int64 v) { $_setInt64(16, v); }
  @$pb.TagNumber(17)
  $core.bool hasBlockRewardLastBlock() => $_has(16);
  @$pb.TagNumber(17)
  void clearBlockRewardLastBlock() => clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get karmaRewardAmount => $_getI64(17);
  @$pb.TagNumber(18)
  set karmaRewardAmount($fixnum.Int64 v) { $_setInt64(17, v); }
  @$pb.TagNumber(18)
  $core.bool hasKarmaRewardAmount() => $_has(17);
  @$pb.TagNumber(18)
  void clearKarmaRewardAmount() => clearField(18);

  @$pb.TagNumber(19)
  $fixnum.Int64 get karmaRewardAlloc => $_getI64(18);
  @$pb.TagNumber(19)
  set karmaRewardAlloc($fixnum.Int64 v) { $_setInt64(18, v); }
  @$pb.TagNumber(19)
  $core.bool hasKarmaRewardAlloc() => $_has(18);
  @$pb.TagNumber(19)
  void clearKarmaRewardAlloc() => clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get karmaRewardTopNUsers => $_getI64(19);
  @$pb.TagNumber(20)
  set karmaRewardTopNUsers($fixnum.Int64 v) { $_setInt64(19, v); }
  @$pb.TagNumber(20)
  $core.bool hasKarmaRewardTopNUsers() => $_has(19);
  @$pb.TagNumber(20)
  void clearKarmaRewardTopNUsers() => clearField(20);

  @$pb.TagNumber(21)
  $fixnum.Int64 get treasuryPremintAmount => $_getI64(20);
  @$pb.TagNumber(21)
  set treasuryPremintAmount($fixnum.Int64 v) { $_setInt64(20, v); }
  @$pb.TagNumber(21)
  $core.bool hasTreasuryPremintAmount() => $_has(20);
  @$pb.TagNumber(21)
  void clearTreasuryPremintAmount() => clearField(21);

  @$pb.TagNumber(22)
  $core.String get treasuryAccountId => $_getSZ(21);
  @$pb.TagNumber(22)
  set treasuryAccountId($core.String v) { $_setString(21, v); }
  @$pb.TagNumber(22)
  $core.bool hasTreasuryAccountId() => $_has(21);
  @$pb.TagNumber(22)
  void clearTreasuryAccountId() => clearField(22);

  @$pb.TagNumber(23)
  $core.String get treasuryAccountName => $_getSZ(22);
  @$pb.TagNumber(23)
  set treasuryAccountName($core.String v) { $_setString(22, v); }
  @$pb.TagNumber(23)
  $core.bool hasTreasuryAccountName() => $_has(22);
  @$pb.TagNumber(23)
  void clearTreasuryAccountName() => clearField(23);

  @$pb.TagNumber(24)
  $core.List<$2.CharTrait> get charTraits => $_getList(23);

  @$pb.TagNumber(25)
  $core.List<$2.PhoneVerifier> get verifiers => $_getList(24);
}

class GetBlockchainDataRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetBlockchainDataRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GetBlockchainDataRequest._() : super();
  factory GetBlockchainDataRequest() => create();
  factory GetBlockchainDataRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetBlockchainDataRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetBlockchainDataRequest clone() => GetBlockchainDataRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetBlockchainDataRequest copyWith(void Function(GetBlockchainDataRequest) updates) => super.copyWith((message) => updates(message as GetBlockchainDataRequest)) as GetBlockchainDataRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetBlockchainDataRequest create() => GetBlockchainDataRequest._();
  GetBlockchainDataRequest createEmptyInstance() => create();
  static $pb.PbList<GetBlockchainDataRequest> createRepeated() => $pb.PbList<GetBlockchainDataRequest>();
  @$core.pragma('dart2js:noInline')
  static GetBlockchainDataRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetBlockchainDataRequest>(create);
  static GetBlockchainDataRequest? _defaultInstance;
}

class GetBlockchainDataResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetBlockchainDataResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.BlockchainStats>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'stats', subBuilder: $2.BlockchainStats.create)
    ..hasRequiredFields = false
  ;

  GetBlockchainDataResponse._() : super();
  factory GetBlockchainDataResponse({
    $2.BlockchainStats? stats,
  }) {
    final _result = create();
    if (stats != null) {
      _result.stats = stats;
    }
    return _result;
  }
  factory GetBlockchainDataResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetBlockchainDataResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetBlockchainDataResponse clone() => GetBlockchainDataResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetBlockchainDataResponse copyWith(void Function(GetBlockchainDataResponse) updates) => super.copyWith((message) => updates(message as GetBlockchainDataResponse)) as GetBlockchainDataResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetBlockchainDataResponse create() => GetBlockchainDataResponse._();
  GetBlockchainDataResponse createEmptyInstance() => create();
  static $pb.PbList<GetBlockchainDataResponse> createRepeated() => $pb.PbList<GetBlockchainDataResponse>();
  @$core.pragma('dart2js:noInline')
  static GetBlockchainDataResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetBlockchainDataResponse>(create);
  static GetBlockchainDataResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $2.BlockchainStats get stats => $_getN(0);
  @$pb.TagNumber(1)
  set stats($2.BlockchainStats v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasStats() => $_has(0);
  @$pb.TagNumber(1)
  void clearStats() => clearField(1);
  @$pb.TagNumber(1)
  $2.BlockchainStats ensureStats() => $_ensure(0);
}

class GetTransactionsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: $2.AccountId.create)
    ..hasRequiredFields = false
  ;

  GetTransactionsRequest._() : super();
  factory GetTransactionsRequest({
    $2.AccountId? accountId,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    return _result;
  }
  factory GetTransactionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionsRequest clone() => GetTransactionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionsRequest copyWith(void Function(GetTransactionsRequest) updates) => super.copyWith((message) => updates(message as GetTransactionsRequest)) as GetTransactionsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetTransactionsRequest create() => GetTransactionsRequest._();
  GetTransactionsRequest createEmptyInstance() => create();
  static $pb.PbList<GetTransactionsRequest> createRepeated() => $pb.PbList<GetTransactionsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionsRequest>(create);
  static GetTransactionsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $2.AccountId get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($2.AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);
  @$pb.TagNumber(1)
  $2.AccountId ensureAccountId() => $_ensure(0);
}

class GetTransactionsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionsResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$2.SignedTransactionWithStatus>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactions', $pb.PbFieldType.PM, subBuilder: $2.SignedTransactionWithStatus.create)
    ..aOM<$2.TransactionEvents>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txEvents', subBuilder: $2.TransactionEvents.create)
    ..hasRequiredFields = false
  ;

  GetTransactionsResponse._() : super();
  factory GetTransactionsResponse({
    $core.Iterable<$2.SignedTransactionWithStatus>? transactions,
    $2.TransactionEvents? txEvents,
  }) {
    final _result = create();
    if (transactions != null) {
      _result.transactions.addAll(transactions);
    }
    if (txEvents != null) {
      _result.txEvents = txEvents;
    }
    return _result;
  }
  factory GetTransactionsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionsResponse clone() => GetTransactionsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionsResponse copyWith(void Function(GetTransactionsResponse) updates) => super.copyWith((message) => updates(message as GetTransactionsResponse)) as GetTransactionsResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetTransactionsResponse create() => GetTransactionsResponse._();
  GetTransactionsResponse createEmptyInstance() => create();
  static $pb.PbList<GetTransactionsResponse> createRepeated() => $pb.PbList<GetTransactionsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionsResponse>(create);
  static GetTransactionsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$2.SignedTransactionWithStatus> get transactions => $_getList(0);

  @$pb.TagNumber(2)
  $2.TransactionEvents get txEvents => $_getN(1);
  @$pb.TagNumber(2)
  set txEvents($2.TransactionEvents v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTxEvents() => $_has(1);
  @$pb.TagNumber(2)
  void clearTxEvents() => clearField(2);
  @$pb.TagNumber(2)
  $2.TransactionEvents ensureTxEvents() => $_ensure(1);
}

class GetTransactionRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txHash', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetTransactionRequest._() : super();
  factory GetTransactionRequest({
    $core.List<$core.int>? txHash,
  }) {
    final _result = create();
    if (txHash != null) {
      _result.txHash = txHash;
    }
    return _result;
  }
  factory GetTransactionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionRequest clone() => GetTransactionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionRequest copyWith(void Function(GetTransactionRequest) updates) => super.copyWith((message) => updates(message as GetTransactionRequest)) as GetTransactionRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetTransactionRequest create() => GetTransactionRequest._();
  GetTransactionRequest createEmptyInstance() => create();
  static $pb.PbList<GetTransactionRequest> createRepeated() => $pb.PbList<GetTransactionRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionRequest>(create);
  static GetTransactionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get txHash => $_getN(0);
  @$pb.TagNumber(1)
  set txHash($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxHash() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxHash() => clearField(1);
}

class GetTransactionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$2.SignedTransactionWithStatus>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transaction', subBuilder: $2.SignedTransactionWithStatus.create)
    ..aOM<$2.TransactionEvents>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txEvents', subBuilder: $2.TransactionEvents.create)
    ..hasRequiredFields = false
  ;

  GetTransactionResponse._() : super();
  factory GetTransactionResponse({
    $2.SignedTransactionWithStatus? transaction,
    $2.TransactionEvents? txEvents,
  }) {
    final _result = create();
    if (transaction != null) {
      _result.transaction = transaction;
    }
    if (txEvents != null) {
      _result.txEvents = txEvents;
    }
    return _result;
  }
  factory GetTransactionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionResponse clone() => GetTransactionResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionResponse copyWith(void Function(GetTransactionResponse) updates) => super.copyWith((message) => updates(message as GetTransactionResponse)) as GetTransactionResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetTransactionResponse create() => GetTransactionResponse._();
  GetTransactionResponse createEmptyInstance() => create();
  static $pb.PbList<GetTransactionResponse> createRepeated() => $pb.PbList<GetTransactionResponse>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionResponse>(create);
  static GetTransactionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $2.SignedTransactionWithStatus get transaction => $_getN(0);
  @$pb.TagNumber(1)
  set transaction($2.SignedTransactionWithStatus v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransaction() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransaction() => clearField(1);
  @$pb.TagNumber(1)
  $2.SignedTransactionWithStatus ensureTransaction() => $_ensure(0);

  @$pb.TagNumber(2)
  $2.TransactionEvents get txEvents => $_getN(1);
  @$pb.TagNumber(2)
  set txEvents($2.TransactionEvents v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTxEvents() => $_has(1);
  @$pb.TagNumber(2)
  void clearTxEvents() => clearField(2);
  @$pb.TagNumber(2)
  $2.TransactionEvents ensureTxEvents() => $_ensure(1);
}

class GetBlockchainEventsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetBlockchainEventsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fromBlockHeight', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'toBlockHeight', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  GetBlockchainEventsRequest._() : super();
  factory GetBlockchainEventsRequest({
    $fixnum.Int64? fromBlockHeight,
    $fixnum.Int64? toBlockHeight,
  }) {
    final _result = create();
    if (fromBlockHeight != null) {
      _result.fromBlockHeight = fromBlockHeight;
    }
    if (toBlockHeight != null) {
      _result.toBlockHeight = toBlockHeight;
    }
    return _result;
  }
  factory GetBlockchainEventsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetBlockchainEventsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetBlockchainEventsRequest clone() => GetBlockchainEventsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetBlockchainEventsRequest copyWith(void Function(GetBlockchainEventsRequest) updates) => super.copyWith((message) => updates(message as GetBlockchainEventsRequest)) as GetBlockchainEventsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetBlockchainEventsRequest create() => GetBlockchainEventsRequest._();
  GetBlockchainEventsRequest createEmptyInstance() => create();
  static $pb.PbList<GetBlockchainEventsRequest> createRepeated() => $pb.PbList<GetBlockchainEventsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetBlockchainEventsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetBlockchainEventsRequest>(create);
  static GetBlockchainEventsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fromBlockHeight => $_getI64(0);
  @$pb.TagNumber(1)
  set fromBlockHeight($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFromBlockHeight() => $_has(0);
  @$pb.TagNumber(1)
  void clearFromBlockHeight() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get toBlockHeight => $_getI64(1);
  @$pb.TagNumber(2)
  set toBlockHeight($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasToBlockHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearToBlockHeight() => clearField(2);
}

class GetBlockchainEventsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetBlockchainEventsResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$2.BlockEvent>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blocksEvents', $pb.PbFieldType.PM, subBuilder: $2.BlockEvent.create)
    ..hasRequiredFields = false
  ;

  GetBlockchainEventsResponse._() : super();
  factory GetBlockchainEventsResponse({
    $core.Iterable<$2.BlockEvent>? blocksEvents,
  }) {
    final _result = create();
    if (blocksEvents != null) {
      _result.blocksEvents.addAll(blocksEvents);
    }
    return _result;
  }
  factory GetBlockchainEventsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetBlockchainEventsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetBlockchainEventsResponse clone() => GetBlockchainEventsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetBlockchainEventsResponse copyWith(void Function(GetBlockchainEventsResponse) updates) => super.copyWith((message) => updates(message as GetBlockchainEventsResponse)) as GetBlockchainEventsResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetBlockchainEventsResponse create() => GetBlockchainEventsResponse._();
  GetBlockchainEventsResponse createEmptyInstance() => create();
  static $pb.PbList<GetBlockchainEventsResponse> createRepeated() => $pb.PbList<GetBlockchainEventsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetBlockchainEventsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetBlockchainEventsResponse>(create);
  static GetBlockchainEventsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$2.BlockEvent> get blocksEvents => $_getList(0);
}

class GetBlocksRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetBlocksRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fromBlockHeight', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'toBlockHeight', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  GetBlocksRequest._() : super();
  factory GetBlocksRequest({
    $fixnum.Int64? fromBlockHeight,
    $fixnum.Int64? toBlockHeight,
  }) {
    final _result = create();
    if (fromBlockHeight != null) {
      _result.fromBlockHeight = fromBlockHeight;
    }
    if (toBlockHeight != null) {
      _result.toBlockHeight = toBlockHeight;
    }
    return _result;
  }
  factory GetBlocksRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetBlocksRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetBlocksRequest clone() => GetBlocksRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetBlocksRequest copyWith(void Function(GetBlocksRequest) updates) => super.copyWith((message) => updates(message as GetBlocksRequest)) as GetBlocksRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetBlocksRequest create() => GetBlocksRequest._();
  GetBlocksRequest createEmptyInstance() => create();
  static $pb.PbList<GetBlocksRequest> createRepeated() => $pb.PbList<GetBlocksRequest>();
  @$core.pragma('dart2js:noInline')
  static GetBlocksRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetBlocksRequest>(create);
  static GetBlocksRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get fromBlockHeight => $_getI64(0);
  @$pb.TagNumber(1)
  set fromBlockHeight($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFromBlockHeight() => $_has(0);
  @$pb.TagNumber(1)
  void clearFromBlockHeight() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get toBlockHeight => $_getI64(1);
  @$pb.TagNumber(2)
  set toBlockHeight($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasToBlockHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearToBlockHeight() => clearField(2);
}

class GetBlocksResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetBlocksResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$2.Block>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blocks', $pb.PbFieldType.PM, subBuilder: $2.Block.create)
    ..hasRequiredFields = false
  ;

  GetBlocksResponse._() : super();
  factory GetBlocksResponse({
    $core.Iterable<$2.Block>? blocks,
  }) {
    final _result = create();
    if (blocks != null) {
      _result.blocks.addAll(blocks);
    }
    return _result;
  }
  factory GetBlocksResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetBlocksResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetBlocksResponse clone() => GetBlocksResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetBlocksResponse copyWith(void Function(GetBlocksResponse) updates) => super.copyWith((message) => updates(message as GetBlocksResponse)) as GetBlocksResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetBlocksResponse create() => GetBlocksResponse._();
  GetBlocksResponse createEmptyInstance() => create();
  static $pb.PbList<GetBlocksResponse> createRepeated() => $pb.PbList<GetBlocksResponse>();
  @$core.pragma('dart2js:noInline')
  static GetBlocksResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetBlocksResponse>(create);
  static GetBlocksResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$2.Block> get blocks => $_getList(0);
}

