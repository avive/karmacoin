///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'types.pb.dart' as $3;

import 'api.pbenum.dart';

export 'api.pbenum.dart';

class SetCommunityAdminRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetCommunityAdminRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$3.AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fromAccountId', subBuilder: $3.AccountId.create)
    ..aOM<$3.AccountId>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'targetAccountId', subBuilder: $3.AccountId.create)
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'communityId', $pb.PbFieldType.OU3)
    ..aOB(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'admin')
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  SetCommunityAdminRequest._() : super();
  factory SetCommunityAdminRequest({
    $3.AccountId? fromAccountId,
    $3.AccountId? targetAccountId,
    $core.int? communityId,
    $core.bool? admin,
    $core.List<$core.int>? signature,
  }) {
    final _result = create();
    if (fromAccountId != null) {
      _result.fromAccountId = fromAccountId;
    }
    if (targetAccountId != null) {
      _result.targetAccountId = targetAccountId;
    }
    if (communityId != null) {
      _result.communityId = communityId;
    }
    if (admin != null) {
      _result.admin = admin;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory SetCommunityAdminRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetCommunityAdminRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetCommunityAdminRequest clone() => SetCommunityAdminRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetCommunityAdminRequest copyWith(void Function(SetCommunityAdminRequest) updates) => super.copyWith((message) => updates(message as SetCommunityAdminRequest)) as SetCommunityAdminRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SetCommunityAdminRequest create() => SetCommunityAdminRequest._();
  SetCommunityAdminRequest createEmptyInstance() => create();
  static $pb.PbList<SetCommunityAdminRequest> createRepeated() => $pb.PbList<SetCommunityAdminRequest>();
  @$core.pragma('dart2js:noInline')
  static SetCommunityAdminRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetCommunityAdminRequest>(create);
  static SetCommunityAdminRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $3.AccountId get fromAccountId => $_getN(0);
  @$pb.TagNumber(1)
  set fromAccountId($3.AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasFromAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFromAccountId() => clearField(1);
  @$pb.TagNumber(1)
  $3.AccountId ensureFromAccountId() => $_ensure(0);

  @$pb.TagNumber(2)
  $3.AccountId get targetAccountId => $_getN(1);
  @$pb.TagNumber(2)
  set targetAccountId($3.AccountId v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTargetAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetAccountId() => clearField(2);
  @$pb.TagNumber(2)
  $3.AccountId ensureTargetAccountId() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.int get communityId => $_getIZ(2);
  @$pb.TagNumber(3)
  set communityId($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasCommunityId() => $_has(2);
  @$pb.TagNumber(3)
  void clearCommunityId() => clearField(3);

  @$pb.TagNumber(4)
  $core.bool get admin => $_getBF(3);
  @$pb.TagNumber(4)
  set admin($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasAdmin() => $_has(3);
  @$pb.TagNumber(4)
  void clearAdmin() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<$core.int> get signature => $_getN(4);
  @$pb.TagNumber(5)
  set signature($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasSignature() => $_has(4);
  @$pb.TagNumber(5)
  void clearSignature() => clearField(5);
}

class SetCommunityAdminResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetCommunityAdminResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  SetCommunityAdminResponse._() : super();
  factory SetCommunityAdminResponse() => create();
  factory SetCommunityAdminResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetCommunityAdminResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetCommunityAdminResponse clone() => SetCommunityAdminResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetCommunityAdminResponse copyWith(void Function(SetCommunityAdminResponse) updates) => super.copyWith((message) => updates(message as SetCommunityAdminResponse)) as SetCommunityAdminResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SetCommunityAdminResponse create() => SetCommunityAdminResponse._();
  SetCommunityAdminResponse createEmptyInstance() => create();
  static $pb.PbList<SetCommunityAdminResponse> createRepeated() => $pb.PbList<SetCommunityAdminResponse>();
  @$core.pragma('dart2js:noInline')
  static SetCommunityAdminResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetCommunityAdminResponse>(create);
  static SetCommunityAdminResponse? _defaultInstance;
}

class GetLeaderBoardRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetLeaderBoardRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GetLeaderBoardRequest._() : super();
  factory GetLeaderBoardRequest() => create();
  factory GetLeaderBoardRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetLeaderBoardRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetLeaderBoardRequest clone() => GetLeaderBoardRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetLeaderBoardRequest copyWith(void Function(GetLeaderBoardRequest) updates) => super.copyWith((message) => updates(message as GetLeaderBoardRequest)) as GetLeaderBoardRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetLeaderBoardRequest create() => GetLeaderBoardRequest._();
  GetLeaderBoardRequest createEmptyInstance() => create();
  static $pb.PbList<GetLeaderBoardRequest> createRepeated() => $pb.PbList<GetLeaderBoardRequest>();
  @$core.pragma('dart2js:noInline')
  static GetLeaderBoardRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetLeaderBoardRequest>(create);
  static GetLeaderBoardRequest? _defaultInstance;
}

class GetContactsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetContactsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'prefix')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'communityId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GetContactsRequest._() : super();
  factory GetContactsRequest({
    $core.String? prefix,
    $core.int? communityId,
  }) {
    final _result = create();
    if (prefix != null) {
      _result.prefix = prefix;
    }
    if (communityId != null) {
      _result.communityId = communityId;
    }
    return _result;
  }
  factory GetContactsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetContactsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetContactsRequest clone() => GetContactsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetContactsRequest copyWith(void Function(GetContactsRequest) updates) => super.copyWith((message) => updates(message as GetContactsRequest)) as GetContactsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetContactsRequest create() => GetContactsRequest._();
  GetContactsRequest createEmptyInstance() => create();
  static $pb.PbList<GetContactsRequest> createRepeated() => $pb.PbList<GetContactsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetContactsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetContactsRequest>(create);
  static GetContactsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get prefix => $_getSZ(0);
  @$pb.TagNumber(1)
  set prefix($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPrefix() => $_has(0);
  @$pb.TagNumber(1)
  void clearPrefix() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get communityId => $_getIZ(1);
  @$pb.TagNumber(2)
  set communityId($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCommunityId() => $_has(1);
  @$pb.TagNumber(2)
  void clearCommunityId() => clearField(2);
}

class GetContactsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetContactsResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$3.Contact>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contacts', $pb.PbFieldType.PM, subBuilder: $3.Contact.create)
    ..hasRequiredFields = false
  ;

  GetContactsResponse._() : super();
  factory GetContactsResponse({
    $core.Iterable<$3.Contact>? contacts,
  }) {
    final _result = create();
    if (contacts != null) {
      _result.contacts.addAll(contacts);
    }
    return _result;
  }
  factory GetContactsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetContactsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetContactsResponse clone() => GetContactsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetContactsResponse copyWith(void Function(GetContactsResponse) updates) => super.copyWith((message) => updates(message as GetContactsResponse)) as GetContactsResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetContactsResponse create() => GetContactsResponse._();
  GetContactsResponse createEmptyInstance() => create();
  static $pb.PbList<GetContactsResponse> createRepeated() => $pb.PbList<GetContactsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetContactsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetContactsResponse>(create);
  static GetContactsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$3.Contact> get contacts => $_getList(0);
}

class GetLeaderBoardResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetLeaderBoardResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$3.LeaderboardEntry>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'leaderboardEntries', $pb.PbFieldType.PM, subBuilder: $3.LeaderboardEntry.create)
    ..hasRequiredFields = false
  ;

  GetLeaderBoardResponse._() : super();
  factory GetLeaderBoardResponse({
    $core.Iterable<$3.LeaderboardEntry>? leaderboardEntries,
  }) {
    final _result = create();
    if (leaderboardEntries != null) {
      _result.leaderboardEntries.addAll(leaderboardEntries);
    }
    return _result;
  }
  factory GetLeaderBoardResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetLeaderBoardResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetLeaderBoardResponse clone() => GetLeaderBoardResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetLeaderBoardResponse copyWith(void Function(GetLeaderBoardResponse) updates) => super.copyWith((message) => updates(message as GetLeaderBoardResponse)) as GetLeaderBoardResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetLeaderBoardResponse create() => GetLeaderBoardResponse._();
  GetLeaderBoardResponse createEmptyInstance() => create();
  static $pb.PbList<GetLeaderBoardResponse> createRepeated() => $pb.PbList<GetLeaderBoardResponse>();
  @$core.pragma('dart2js:noInline')
  static GetLeaderBoardResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetLeaderBoardResponse>(create);
  static GetLeaderBoardResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$3.LeaderboardEntry> get leaderboardEntries => $_getList(0);
}

class GetTransactionsFromHashesRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionsFromHashesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txHashes', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  GetTransactionsFromHashesRequest._() : super();
  factory GetTransactionsFromHashesRequest({
    $core.Iterable<$core.List<$core.int>>? txHashes,
  }) {
    final _result = create();
    if (txHashes != null) {
      _result.txHashes.addAll(txHashes);
    }
    return _result;
  }
  factory GetTransactionsFromHashesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionsFromHashesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionsFromHashesRequest clone() => GetTransactionsFromHashesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionsFromHashesRequest copyWith(void Function(GetTransactionsFromHashesRequest) updates) => super.copyWith((message) => updates(message as GetTransactionsFromHashesRequest)) as GetTransactionsFromHashesRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetTransactionsFromHashesRequest create() => GetTransactionsFromHashesRequest._();
  GetTransactionsFromHashesRequest createEmptyInstance() => create();
  static $pb.PbList<GetTransactionsFromHashesRequest> createRepeated() => $pb.PbList<GetTransactionsFromHashesRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionsFromHashesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionsFromHashesRequest>(create);
  static GetTransactionsFromHashesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get txHashes => $_getList(0);
}

class GetTransactionsFromHashesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionsFromHashesResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$3.SignedTransactionWithStatus>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactions', $pb.PbFieldType.PM, subBuilder: $3.SignedTransactionWithStatus.create)
    ..aOM<$3.TransactionEvents>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txEvents', subBuilder: $3.TransactionEvents.create)
    ..hasRequiredFields = false
  ;

  GetTransactionsFromHashesResponse._() : super();
  factory GetTransactionsFromHashesResponse({
    $core.Iterable<$3.SignedTransactionWithStatus>? transactions,
    $3.TransactionEvents? txEvents,
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
  factory GetTransactionsFromHashesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionsFromHashesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionsFromHashesResponse clone() => GetTransactionsFromHashesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionsFromHashesResponse copyWith(void Function(GetTransactionsFromHashesResponse) updates) => super.copyWith((message) => updates(message as GetTransactionsFromHashesResponse)) as GetTransactionsFromHashesResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetTransactionsFromHashesResponse create() => GetTransactionsFromHashesResponse._();
  GetTransactionsFromHashesResponse createEmptyInstance() => create();
  static $pb.PbList<GetTransactionsFromHashesResponse> createRepeated() => $pb.PbList<GetTransactionsFromHashesResponse>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionsFromHashesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionsFromHashesResponse>(create);
  static GetTransactionsFromHashesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$3.SignedTransactionWithStatus> get transactions => $_getList(0);

  @$pb.TagNumber(2)
  $3.TransactionEvents get txEvents => $_getN(1);
  @$pb.TagNumber(2)
  set txEvents($3.TransactionEvents v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTxEvents() => $_has(1);
  @$pb.TagNumber(2)
  void clearTxEvents() => clearField(2);
  @$pb.TagNumber(2)
  $3.TransactionEvents ensureTxEvents() => $_ensure(1);
}

class GetAllUsersRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetAllUsersRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'communityId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  GetAllUsersRequest._() : super();
  factory GetAllUsersRequest({
    $core.int? communityId,
  }) {
    final _result = create();
    if (communityId != null) {
      _result.communityId = communityId;
    }
    return _result;
  }
  factory GetAllUsersRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAllUsersRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAllUsersRequest clone() => GetAllUsersRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAllUsersRequest copyWith(void Function(GetAllUsersRequest) updates) => super.copyWith((message) => updates(message as GetAllUsersRequest)) as GetAllUsersRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetAllUsersRequest create() => GetAllUsersRequest._();
  GetAllUsersRequest createEmptyInstance() => create();
  static $pb.PbList<GetAllUsersRequest> createRepeated() => $pb.PbList<GetAllUsersRequest>();
  @$core.pragma('dart2js:noInline')
  static GetAllUsersRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetAllUsersRequest>(create);
  static GetAllUsersRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get communityId => $_getIZ(0);
  @$pb.TagNumber(1)
  set communityId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCommunityId() => $_has(0);
  @$pb.TagNumber(1)
  void clearCommunityId() => clearField(1);
}

class GetAllUsersResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetAllUsersResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$3.User>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'users', $pb.PbFieldType.PM, subBuilder: $3.User.create)
    ..hasRequiredFields = false
  ;

  GetAllUsersResponse._() : super();
  factory GetAllUsersResponse({
    $core.Iterable<$3.User>? users,
  }) {
    final _result = create();
    if (users != null) {
      _result.users.addAll(users);
    }
    return _result;
  }
  factory GetAllUsersResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAllUsersResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAllUsersResponse clone() => GetAllUsersResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAllUsersResponse copyWith(void Function(GetAllUsersResponse) updates) => super.copyWith((message) => updates(message as GetAllUsersResponse)) as GetAllUsersResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetAllUsersResponse create() => GetAllUsersResponse._();
  GetAllUsersResponse createEmptyInstance() => create();
  static $pb.PbList<GetAllUsersResponse> createRepeated() => $pb.PbList<GetAllUsersResponse>();
  @$core.pragma('dart2js:noInline')
  static GetAllUsersResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetAllUsersResponse>(create);
  static GetAllUsersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$3.User> get users => $_getList(0);
}

class GetExchangeRateRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetExchangeRateRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  GetExchangeRateRequest._() : super();
  factory GetExchangeRateRequest() => create();
  factory GetExchangeRateRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetExchangeRateRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetExchangeRateRequest clone() => GetExchangeRateRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetExchangeRateRequest copyWith(void Function(GetExchangeRateRequest) updates) => super.copyWith((message) => updates(message as GetExchangeRateRequest)) as GetExchangeRateRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetExchangeRateRequest create() => GetExchangeRateRequest._();
  GetExchangeRateRequest createEmptyInstance() => create();
  static $pb.PbList<GetExchangeRateRequest> createRepeated() => $pb.PbList<GetExchangeRateRequest>();
  @$core.pragma('dart2js:noInline')
  static GetExchangeRateRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetExchangeRateRequest>(create);
  static GetExchangeRateRequest? _defaultInstance;
}

class GetExchangeRateResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetExchangeRateResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..a<$core.double>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'exchangeRate', $pb.PbFieldType.OD)
    ..hasRequiredFields = false
  ;

  GetExchangeRateResponse._() : super();
  factory GetExchangeRateResponse({
    $core.double? exchangeRate,
  }) {
    final _result = create();
    if (exchangeRate != null) {
      _result.exchangeRate = exchangeRate;
    }
    return _result;
  }
  factory GetExchangeRateResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetExchangeRateResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetExchangeRateResponse clone() => GetExchangeRateResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetExchangeRateResponse copyWith(void Function(GetExchangeRateResponse) updates) => super.copyWith((message) => updates(message as GetExchangeRateResponse)) as GetExchangeRateResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetExchangeRateResponse create() => GetExchangeRateResponse._();
  GetExchangeRateResponse createEmptyInstance() => create();
  static $pb.PbList<GetExchangeRateResponse> createRepeated() => $pb.PbList<GetExchangeRateResponse>();
  @$core.pragma('dart2js:noInline')
  static GetExchangeRateResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetExchangeRateResponse>(create);
  static GetExchangeRateResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.double get exchangeRate => $_getN(0);
  @$pb.TagNumber(1)
  set exchangeRate($core.double v) { $_setDouble(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasExchangeRate() => $_has(0);
  @$pb.TagNumber(1)
  void clearExchangeRate() => clearField(1);
}

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
    ..aOM<$3.User>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', subBuilder: $3.User.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByUserNameResponse._() : super();
  factory GetUserInfoByUserNameResponse({
    $3.User? user,
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
  $3.User get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($3.User v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);
  @$pb.TagNumber(1)
  $3.User ensureUser() => $_ensure(0);
}

class SubmitTransactionRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SubmitTransactionRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$3.SignedTransaction>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transaction', subBuilder: $3.SignedTransaction.create)
    ..hasRequiredFields = false
  ;

  SubmitTransactionRequest._() : super();
  factory SubmitTransactionRequest({
    $3.SignedTransaction? transaction,
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
  $3.SignedTransaction get transaction => $_getN(0);
  @$pb.TagNumber(1)
  set transaction($3.SignedTransaction v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransaction() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransaction() => clearField(1);
  @$pb.TagNumber(1)
  $3.SignedTransaction ensureTransaction() => $_ensure(0);
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
    ..aOM<$3.MobileNumber>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: $3.MobileNumber.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByNumberRequest._() : super();
  factory GetUserInfoByNumberRequest({
    $3.MobileNumber? mobileNumber,
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
  $3.MobileNumber get mobileNumber => $_getN(0);
  @$pb.TagNumber(1)
  set mobileNumber($3.MobileNumber v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasMobileNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearMobileNumber() => clearField(1);
  @$pb.TagNumber(1)
  $3.MobileNumber ensureMobileNumber() => $_ensure(0);
}

class GetUserInfoByNumberResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByNumberResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$3.User>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', subBuilder: $3.User.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByNumberResponse._() : super();
  factory GetUserInfoByNumberResponse({
    $3.User? user,
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
  $3.User get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($3.User v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);
  @$pb.TagNumber(1)
  $3.User ensureUser() => $_ensure(0);
}

class GetUserInfoByAccountRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByAccountRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$3.AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: $3.AccountId.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByAccountRequest._() : super();
  factory GetUserInfoByAccountRequest({
    $3.AccountId? accountId,
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
  $3.AccountId get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($3.AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);
  @$pb.TagNumber(1)
  $3.AccountId ensureAccountId() => $_ensure(0);
}

class GetUserInfoByAccountResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetUserInfoByAccountResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$3.User>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'user', subBuilder: $3.User.create)
    ..hasRequiredFields = false
  ;

  GetUserInfoByAccountResponse._() : super();
  factory GetUserInfoByAccountResponse({
    $3.User? user,
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
  $3.User get user => $_getN(0);
  @$pb.TagNumber(1)
  set user($3.User v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasUser() => $_has(0);
  @$pb.TagNumber(1)
  void clearUser() => clearField(1);
  @$pb.TagNumber(1)
  $3.User ensureUser() => $_ensure(0);
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
    ..aOM<$3.GenesisData>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'genesisData', subBuilder: $3.GenesisData.create)
    ..hasRequiredFields = false
  ;

  GetGenesisDataResponse._() : super();
  factory GetGenesisDataResponse({
    $3.GenesisData? genesisData,
  }) {
    final _result = create();
    if (genesisData != null) {
      _result.genesisData = genesisData;
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
  $3.GenesisData get genesisData => $_getN(0);
  @$pb.TagNumber(1)
  set genesisData($3.GenesisData v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasGenesisData() => $_has(0);
  @$pb.TagNumber(1)
  void clearGenesisData() => clearField(1);
  @$pb.TagNumber(1)
  $3.GenesisData ensureGenesisData() => $_ensure(0);
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
    ..aOM<$3.BlockchainStats>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'stats', subBuilder: $3.BlockchainStats.create)
    ..hasRequiredFields = false
  ;

  GetBlockchainDataResponse._() : super();
  factory GetBlockchainDataResponse({
    $3.BlockchainStats? stats,
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
  $3.BlockchainStats get stats => $_getN(0);
  @$pb.TagNumber(1)
  set stats($3.BlockchainStats v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasStats() => $_has(0);
  @$pb.TagNumber(1)
  void clearStats() => clearField(1);
  @$pb.TagNumber(1)
  $3.BlockchainStats ensureStats() => $_ensure(0);
}

class GetTransactionsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..aOM<$3.AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: $3.AccountId.create)
    ..hasRequiredFields = false
  ;

  GetTransactionsRequest._() : super();
  factory GetTransactionsRequest({
    $3.AccountId? accountId,
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
  $3.AccountId get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($3.AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);
  @$pb.TagNumber(1)
  $3.AccountId ensureAccountId() => $_ensure(0);
}

class GetTransactionsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionsResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.api'), createEmptyInstance: create)
    ..pc<$3.SignedTransactionWithStatus>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactions', $pb.PbFieldType.PM, subBuilder: $3.SignedTransactionWithStatus.create)
    ..aOM<$3.TransactionEvents>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txEvents', subBuilder: $3.TransactionEvents.create)
    ..hasRequiredFields = false
  ;

  GetTransactionsResponse._() : super();
  factory GetTransactionsResponse({
    $core.Iterable<$3.SignedTransactionWithStatus>? transactions,
    $3.TransactionEvents? txEvents,
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
  $core.List<$3.SignedTransactionWithStatus> get transactions => $_getList(0);

  @$pb.TagNumber(2)
  $3.TransactionEvents get txEvents => $_getN(1);
  @$pb.TagNumber(2)
  set txEvents($3.TransactionEvents v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTxEvents() => $_has(1);
  @$pb.TagNumber(2)
  void clearTxEvents() => clearField(2);
  @$pb.TagNumber(2)
  $3.TransactionEvents ensureTxEvents() => $_ensure(1);
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
    ..aOM<$3.SignedTransactionWithStatus>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transaction', subBuilder: $3.SignedTransactionWithStatus.create)
    ..aOM<$3.TransactionEvents>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txEvents', subBuilder: $3.TransactionEvents.create)
    ..hasRequiredFields = false
  ;

  GetTransactionResponse._() : super();
  factory GetTransactionResponse({
    $3.SignedTransactionWithStatus? transaction,
    $3.TransactionEvents? txEvents,
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
  $3.SignedTransactionWithStatus get transaction => $_getN(0);
  @$pb.TagNumber(1)
  set transaction($3.SignedTransactionWithStatus v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransaction() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransaction() => clearField(1);
  @$pb.TagNumber(1)
  $3.SignedTransactionWithStatus ensureTransaction() => $_ensure(0);

  @$pb.TagNumber(2)
  $3.TransactionEvents get txEvents => $_getN(1);
  @$pb.TagNumber(2)
  set txEvents($3.TransactionEvents v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTxEvents() => $_has(1);
  @$pb.TagNumber(2)
  void clearTxEvents() => clearField(2);
  @$pb.TagNumber(2)
  $3.TransactionEvents ensureTxEvents() => $_ensure(1);
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
    ..pc<$3.BlockEvent>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blocksEvents', $pb.PbFieldType.PM, subBuilder: $3.BlockEvent.create)
    ..hasRequiredFields = false
  ;

  GetBlockchainEventsResponse._() : super();
  factory GetBlockchainEventsResponse({
    $core.Iterable<$3.BlockEvent>? blocksEvents,
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
  $core.List<$3.BlockEvent> get blocksEvents => $_getList(0);
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
    ..pc<$3.Block>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blocks', $pb.PbFieldType.PM, subBuilder: $3.Block.create)
    ..hasRequiredFields = false
  ;

  GetBlocksResponse._() : super();
  factory GetBlocksResponse({
    $core.Iterable<$3.Block>? blocks,
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
  $core.List<$3.Block> get blocks => $_getList(0);
}

