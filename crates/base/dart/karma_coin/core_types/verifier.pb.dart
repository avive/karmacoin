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

