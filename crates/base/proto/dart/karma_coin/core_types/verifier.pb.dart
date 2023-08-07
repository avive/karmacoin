///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/verifier.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'types.pb.dart' as $3;

class VerifierInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'VerifierInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.verifier'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOM<$3.AccountId>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: $3.AccountId.create)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verifierEndpointIp4')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verifierEndpointIp6')
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'apiEndpointIp4')
    ..aOS(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'apiEndpointIp6')
    ..aOM<$3.Signature>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', subBuilder: $3.Signature.create)
    ..hasRequiredFields = false
  ;

  VerifierInfo._() : super();
  factory VerifierInfo({
    $core.String? name,
    $3.AccountId? accountId,
    $core.String? verifierEndpointIp4,
    $core.String? verifierEndpointIp6,
    $core.String? apiEndpointIp4,
    $core.String? apiEndpointIp6,
    $3.Signature? signature,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (verifierEndpointIp4 != null) {
      _result.verifierEndpointIp4 = verifierEndpointIp4;
    }
    if (verifierEndpointIp6 != null) {
      _result.verifierEndpointIp6 = verifierEndpointIp6;
    }
    if (apiEndpointIp4 != null) {
      _result.apiEndpointIp4 = apiEndpointIp4;
    }
    if (apiEndpointIp6 != null) {
      _result.apiEndpointIp6 = apiEndpointIp6;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory VerifierInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory VerifierInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  VerifierInfo clone() => VerifierInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  VerifierInfo copyWith(void Function(VerifierInfo) updates) => super.copyWith((message) => updates(message as VerifierInfo)) as VerifierInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static VerifierInfo create() => VerifierInfo._();
  VerifierInfo createEmptyInstance() => create();
  static $pb.PbList<VerifierInfo> createRepeated() => $pb.PbList<VerifierInfo>();
  @$core.pragma('dart2js:noInline')
  static VerifierInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerifierInfo>(create);
  static VerifierInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(2)
  $3.AccountId get accountId => $_getN(1);
  @$pb.TagNumber(2)
  set accountId($3.AccountId v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountId() => clearField(2);
  @$pb.TagNumber(2)
  $3.AccountId ensureAccountId() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get verifierEndpointIp4 => $_getSZ(2);
  @$pb.TagNumber(3)
  set verifierEndpointIp4($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasVerifierEndpointIp4() => $_has(2);
  @$pb.TagNumber(3)
  void clearVerifierEndpointIp4() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get verifierEndpointIp6 => $_getSZ(3);
  @$pb.TagNumber(4)
  set verifierEndpointIp6($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasVerifierEndpointIp6() => $_has(3);
  @$pb.TagNumber(4)
  void clearVerifierEndpointIp6() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get apiEndpointIp4 => $_getSZ(4);
  @$pb.TagNumber(5)
  set apiEndpointIp4($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasApiEndpointIp4() => $_has(4);
  @$pb.TagNumber(5)
  void clearApiEndpointIp4() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get apiEndpointIp6 => $_getSZ(5);
  @$pb.TagNumber(6)
  set apiEndpointIp6($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasApiEndpointIp6() => $_has(5);
  @$pb.TagNumber(6)
  void clearApiEndpointIp6() => clearField(6);

  @$pb.TagNumber(7)
  $3.Signature get signature => $_getN(6);
  @$pb.TagNumber(7)
  set signature($3.Signature v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasSignature() => $_has(6);
  @$pb.TagNumber(7)
  void clearSignature() => clearField(7);
  @$pb.TagNumber(7)
  $3.Signature ensureSignature() => $_ensure(6);
}

class VerifyNumberRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'VerifyNumberRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.verifier'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<$3.AccountId>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: $3.AccountId.create)
    ..aOM<$3.MobileNumber>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: $3.MobileNumber.create)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'requestedUserName')
    ..aOM<$3.Signature>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', subBuilder: $3.Signature.create)
    ..hasRequiredFields = false
  ;

  VerifyNumberRequest._() : super();
  factory VerifyNumberRequest({
    $fixnum.Int64? timestamp,
    $3.AccountId? accountId,
    $3.MobileNumber? mobileNumber,
    $core.String? requestedUserName,
    $3.Signature? signature,
  }) {
    final _result = create();
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (mobileNumber != null) {
      _result.mobileNumber = mobileNumber;
    }
    if (requestedUserName != null) {
      _result.requestedUserName = requestedUserName;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory VerifyNumberRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory VerifyNumberRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  VerifyNumberRequest clone() => VerifyNumberRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  VerifyNumberRequest copyWith(void Function(VerifyNumberRequest) updates) => super.copyWith((message) => updates(message as VerifyNumberRequest)) as VerifyNumberRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static VerifyNumberRequest create() => VerifyNumberRequest._();
  VerifyNumberRequest createEmptyInstance() => create();
  static $pb.PbList<VerifyNumberRequest> createRepeated() => $pb.PbList<VerifyNumberRequest>();
  @$core.pragma('dart2js:noInline')
  static VerifyNumberRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerifyNumberRequest>(create);
  static VerifyNumberRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get timestamp => $_getI64(0);
  @$pb.TagNumber(1)
  set timestamp($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => clearField(1);

  @$pb.TagNumber(2)
  $3.AccountId get accountId => $_getN(1);
  @$pb.TagNumber(2)
  set accountId($3.AccountId v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountId() => clearField(2);
  @$pb.TagNumber(2)
  $3.AccountId ensureAccountId() => $_ensure(1);

  @$pb.TagNumber(3)
  $3.MobileNumber get mobileNumber => $_getN(2);
  @$pb.TagNumber(3)
  set mobileNumber($3.MobileNumber v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasMobileNumber() => $_has(2);
  @$pb.TagNumber(3)
  void clearMobileNumber() => clearField(3);
  @$pb.TagNumber(3)
  $3.MobileNumber ensureMobileNumber() => $_ensure(2);

  @$pb.TagNumber(4)
  $core.String get requestedUserName => $_getSZ(3);
  @$pb.TagNumber(4)
  set requestedUserName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasRequestedUserName() => $_has(3);
  @$pb.TagNumber(4)
  void clearRequestedUserName() => clearField(4);

  @$pb.TagNumber(5)
  $3.Signature get signature => $_getN(4);
  @$pb.TagNumber(5)
  set signature($3.Signature v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasSignature() => $_has(4);
  @$pb.TagNumber(5)
  void clearSignature() => clearField(5);
  @$pb.TagNumber(5)
  $3.Signature ensureSignature() => $_ensure(4);
}

class VerifyNumberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'VerifyNumberResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.verifier'), createEmptyInstance: create)
    ..aOM<$3.UserVerificationData>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userVerificationData', subBuilder: $3.UserVerificationData.create)
    ..hasRequiredFields = false
  ;

  VerifyNumberResponse._() : super();
  factory VerifyNumberResponse({
    $3.UserVerificationData? userVerificationData,
  }) {
    final _result = create();
    if (userVerificationData != null) {
      _result.userVerificationData = userVerificationData;
    }
    return _result;
  }
  factory VerifyNumberResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory VerifyNumberResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  VerifyNumberResponse clone() => VerifyNumberResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  VerifyNumberResponse copyWith(void Function(VerifyNumberResponse) updates) => super.copyWith((message) => updates(message as VerifyNumberResponse)) as VerifyNumberResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static VerifyNumberResponse create() => VerifyNumberResponse._();
  VerifyNumberResponse createEmptyInstance() => create();
  static $pb.PbList<VerifyNumberResponse> createRepeated() => $pb.PbList<VerifyNumberResponse>();
  @$core.pragma('dart2js:noInline')
  static VerifyNumberResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerifyNumberResponse>(create);
  static VerifyNumberResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $3.UserVerificationData get userVerificationData => $_getN(0);
  @$pb.TagNumber(1)
  set userVerificationData($3.UserVerificationData v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUserVerificationData() => $_has(0);
  @$pb.TagNumber(1)
  void clearUserVerificationData() => clearField(1);
  @$pb.TagNumber(1)
  $3.UserVerificationData ensureUserVerificationData() => $_ensure(0);
}

class VerifyNumberRequestDataEx extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'VerifyNumberRequestDataEx', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.verifier'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<$3.AccountId>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: $3.AccountId.create)
    ..aOM<$3.MobileNumber>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: $3.MobileNumber.create)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'requestedUserName')
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'bypassToken', $pb.PbFieldType.OY)
    ..aOS(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verificationCode')
    ..aOS(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verificationSid')
    ..hasRequiredFields = false
  ;

  VerifyNumberRequestDataEx._() : super();
  factory VerifyNumberRequestDataEx({
    $fixnum.Int64? timestamp,
    $3.AccountId? accountId,
    $3.MobileNumber? mobileNumber,
    $core.String? requestedUserName,
    $core.List<$core.int>? bypassToken,
    $core.String? verificationCode,
    $core.String? verificationSid,
  }) {
    final _result = create();
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (mobileNumber != null) {
      _result.mobileNumber = mobileNumber;
    }
    if (requestedUserName != null) {
      _result.requestedUserName = requestedUserName;
    }
    if (bypassToken != null) {
      _result.bypassToken = bypassToken;
    }
    if (verificationCode != null) {
      _result.verificationCode = verificationCode;
    }
    if (verificationSid != null) {
      _result.verificationSid = verificationSid;
    }
    return _result;
  }
  factory VerifyNumberRequestDataEx.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory VerifyNumberRequestDataEx.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  VerifyNumberRequestDataEx clone() => VerifyNumberRequestDataEx()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  VerifyNumberRequestDataEx copyWith(void Function(VerifyNumberRequestDataEx) updates) => super.copyWith((message) => updates(message as VerifyNumberRequestDataEx)) as VerifyNumberRequestDataEx; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static VerifyNumberRequestDataEx create() => VerifyNumberRequestDataEx._();
  VerifyNumberRequestDataEx createEmptyInstance() => create();
  static $pb.PbList<VerifyNumberRequestDataEx> createRepeated() => $pb.PbList<VerifyNumberRequestDataEx>();
  @$core.pragma('dart2js:noInline')
  static VerifyNumberRequestDataEx getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerifyNumberRequestDataEx>(create);
  static VerifyNumberRequestDataEx? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get timestamp => $_getI64(0);
  @$pb.TagNumber(1)
  set timestamp($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => clearField(1);

  @$pb.TagNumber(2)
  $3.AccountId get accountId => $_getN(1);
  @$pb.TagNumber(2)
  set accountId($3.AccountId v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountId() => clearField(2);
  @$pb.TagNumber(2)
  $3.AccountId ensureAccountId() => $_ensure(1);

  @$pb.TagNumber(3)
  $3.MobileNumber get mobileNumber => $_getN(2);
  @$pb.TagNumber(3)
  set mobileNumber($3.MobileNumber v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasMobileNumber() => $_has(2);
  @$pb.TagNumber(3)
  void clearMobileNumber() => clearField(3);
  @$pb.TagNumber(3)
  $3.MobileNumber ensureMobileNumber() => $_ensure(2);

  @$pb.TagNumber(4)
  $core.String get requestedUserName => $_getSZ(3);
  @$pb.TagNumber(4)
  set requestedUserName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasRequestedUserName() => $_has(3);
  @$pb.TagNumber(4)
  void clearRequestedUserName() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.int> get bypassToken => $_getN(4);
  @$pb.TagNumber(5)
  set bypassToken($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasBypassToken() => $_has(4);
  @$pb.TagNumber(5)
  void clearBypassToken() => clearField(5);

  @$pb.TagNumber(6)
  $core.String get verificationCode => $_getSZ(5);
  @$pb.TagNumber(6)
  set verificationCode($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasVerificationCode() => $_has(5);
  @$pb.TagNumber(6)
  void clearVerificationCode() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get verificationSid => $_getSZ(6);
  @$pb.TagNumber(7)
  set verificationSid($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasVerificationSid() => $_has(6);
  @$pb.TagNumber(7)
  void clearVerificationSid() => clearField(7);
}

class VerifyNumberRequestEx extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'VerifyNumberRequestEx', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.verifier'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  VerifyNumberRequestEx._() : super();
  factory VerifyNumberRequestEx({
    $core.List<$core.int>? data,
    $core.List<$core.int>? signature,
  }) {
    final _result = create();
    if (data != null) {
      _result.data = data;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory VerifyNumberRequestEx.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory VerifyNumberRequestEx.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  VerifyNumberRequestEx clone() => VerifyNumberRequestEx()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  VerifyNumberRequestEx copyWith(void Function(VerifyNumberRequestEx) updates) => super.copyWith((message) => updates(message as VerifyNumberRequestEx)) as VerifyNumberRequestEx; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static VerifyNumberRequestEx create() => VerifyNumberRequestEx._();
  VerifyNumberRequestEx createEmptyInstance() => create();
  static $pb.PbList<VerifyNumberRequestEx> createRepeated() => $pb.PbList<VerifyNumberRequestEx>();
  @$core.pragma('dart2js:noInline')
  static VerifyNumberRequestEx getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<VerifyNumberRequestEx>(create);
  static VerifyNumberRequestEx? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => clearField(2);
}

class SmsInviteMetadata extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SmsInviteMetadata', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.verifier'), createEmptyInstance: create)
    ..aOM<$3.MobileNumber>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: $3.MobileNumber.create)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastMessageSentTimeStamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'messagesSent', $pb.PbFieldType.OU3)
    ..aOM<$3.AccountId>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'inviterAccountId', subBuilder: $3.AccountId.create)
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'inviteTxHash', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  SmsInviteMetadata._() : super();
  factory SmsInviteMetadata({
    $3.MobileNumber? mobileNumber,
    $fixnum.Int64? lastMessageSentTimeStamp,
    $core.int? messagesSent,
    $3.AccountId? inviterAccountId,
    $core.List<$core.int>? inviteTxHash,
  }) {
    final _result = create();
    if (mobileNumber != null) {
      _result.mobileNumber = mobileNumber;
    }
    if (lastMessageSentTimeStamp != null) {
      _result.lastMessageSentTimeStamp = lastMessageSentTimeStamp;
    }
    if (messagesSent != null) {
      _result.messagesSent = messagesSent;
    }
    if (inviterAccountId != null) {
      _result.inviterAccountId = inviterAccountId;
    }
    if (inviteTxHash != null) {
      _result.inviteTxHash = inviteTxHash;
    }
    return _result;
  }
  factory SmsInviteMetadata.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SmsInviteMetadata.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SmsInviteMetadata clone() => SmsInviteMetadata()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SmsInviteMetadata copyWith(void Function(SmsInviteMetadata) updates) => super.copyWith((message) => updates(message as SmsInviteMetadata)) as SmsInviteMetadata; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SmsInviteMetadata create() => SmsInviteMetadata._();
  SmsInviteMetadata createEmptyInstance() => create();
  static $pb.PbList<SmsInviteMetadata> createRepeated() => $pb.PbList<SmsInviteMetadata>();
  @$core.pragma('dart2js:noInline')
  static SmsInviteMetadata getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SmsInviteMetadata>(create);
  static SmsInviteMetadata? _defaultInstance;

  @$pb.TagNumber(1)
  $3.MobileNumber get mobileNumber => $_getN(0);
  @$pb.TagNumber(1)
  set mobileNumber($3.MobileNumber v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasMobileNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearMobileNumber() => clearField(1);
  @$pb.TagNumber(1)
  $3.MobileNumber ensureMobileNumber() => $_ensure(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get lastMessageSentTimeStamp => $_getI64(1);
  @$pb.TagNumber(2)
  set lastMessageSentTimeStamp($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastMessageSentTimeStamp() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastMessageSentTimeStamp() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get messagesSent => $_getIZ(2);
  @$pb.TagNumber(3)
  set messagesSent($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMessagesSent() => $_has(2);
  @$pb.TagNumber(3)
  void clearMessagesSent() => clearField(3);

  @$pb.TagNumber(4)
  $3.AccountId get inviterAccountId => $_getN(3);
  @$pb.TagNumber(4)
  set inviterAccountId($3.AccountId v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasInviterAccountId() => $_has(3);
  @$pb.TagNumber(4)
  void clearInviterAccountId() => clearField(4);
  @$pb.TagNumber(4)
  $3.AccountId ensureInviterAccountId() => $_ensure(3);

  @$pb.TagNumber(5)
  $core.List<$core.int> get inviteTxHash => $_getN(4);
  @$pb.TagNumber(5)
  set inviteTxHash($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasInviteTxHash() => $_has(4);
  @$pb.TagNumber(5)
  void clearInviteTxHash() => clearField(5);
}

