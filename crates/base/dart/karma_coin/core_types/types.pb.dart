///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/types.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'types.pbenum.dart';

export 'types.pbenum.dart';

class AccountId extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AccountId', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  AccountId._() : super();
  factory AccountId({
    $core.List<$core.int>? data,
  }) {
    final _result = create();
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory AccountId.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountId.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountId clone() => AccountId()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountId copyWith(void Function(AccountId) updates) => super.copyWith((message) => updates(message as AccountId)) as AccountId; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AccountId create() => AccountId._();
  AccountId createEmptyInstance() => create();
  static $pb.PbList<AccountId> createRepeated() => $pb.PbList<AccountId>();
  @$core.pragma('dart2js:noInline')
  static AccountId getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountId>(create);
  static AccountId? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
}

class Balance extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Balance', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'free', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'reserved', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'miscFrozen', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feeFrozen', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  Balance._() : super();
  factory Balance({
    $fixnum.Int64? free,
    $fixnum.Int64? reserved,
    $fixnum.Int64? miscFrozen,
    $fixnum.Int64? feeFrozen,
  }) {
    final _result = create();
    if (free != null) {
      _result.free = free;
    }
    if (reserved != null) {
      _result.reserved = reserved;
    }
    if (miscFrozen != null) {
      _result.miscFrozen = miscFrozen;
    }
    if (feeFrozen != null) {
      _result.feeFrozen = feeFrozen;
    }
    return _result;
  }
  factory Balance.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Balance.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Balance clone() => Balance()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Balance copyWith(void Function(Balance) updates) => super.copyWith((message) => updates(message as Balance)) as Balance; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Balance create() => Balance._();
  Balance createEmptyInstance() => create();
  static $pb.PbList<Balance> createRepeated() => $pb.PbList<Balance>();
  @$core.pragma('dart2js:noInline')
  static Balance getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Balance>(create);
  static Balance? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get free => $_getI64(0);
  @$pb.TagNumber(1)
  set free($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFree() => $_has(0);
  @$pb.TagNumber(1)
  void clearFree() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get reserved => $_getI64(1);
  @$pb.TagNumber(2)
  set reserved($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasReserved() => $_has(1);
  @$pb.TagNumber(2)
  void clearReserved() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get miscFrozen => $_getI64(2);
  @$pb.TagNumber(3)
  set miscFrozen($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMiscFrozen() => $_has(2);
  @$pb.TagNumber(3)
  void clearMiscFrozen() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get feeFrozen => $_getI64(3);
  @$pb.TagNumber(4)
  set feeFrozen($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFeeFrozen() => $_has(3);
  @$pb.TagNumber(4)
  void clearFeeFrozen() => clearField(4);
}

class PublicKey extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'PublicKey', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'key', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  PublicKey._() : super();
  factory PublicKey({
    $core.List<$core.int>? key,
  }) {
    final _result = create();
    if (key != null) {
      _result.key = key;
    }
    return _result;
  }
  factory PublicKey.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PublicKey.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PublicKey clone() => PublicKey()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PublicKey copyWith(void Function(PublicKey) updates) => super.copyWith((message) => updates(message as PublicKey)) as PublicKey; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PublicKey create() => PublicKey._();
  PublicKey createEmptyInstance() => create();
  static $pb.PbList<PublicKey> createRepeated() => $pb.PbList<PublicKey>();
  @$core.pragma('dart2js:noInline')
  static PublicKey getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PublicKey>(create);
  static PublicKey? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get key => $_getN(0);
  @$pb.TagNumber(1)
  set key($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => clearField(1);
}

class PrivateKey extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'PrivateKey', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'key', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  PrivateKey._() : super();
  factory PrivateKey({
    $core.List<$core.int>? key,
  }) {
    final _result = create();
    if (key != null) {
      _result.key = key;
    }
    return _result;
  }
  factory PrivateKey.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PrivateKey.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PrivateKey clone() => PrivateKey()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PrivateKey copyWith(void Function(PrivateKey) updates) => super.copyWith((message) => updates(message as PrivateKey)) as PrivateKey; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PrivateKey create() => PrivateKey._();
  PrivateKey createEmptyInstance() => create();
  static $pb.PbList<PrivateKey> createRepeated() => $pb.PbList<PrivateKey>();
  @$core.pragma('dart2js:noInline')
  static PrivateKey getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PrivateKey>(create);
  static PrivateKey? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get key => $_getN(0);
  @$pb.TagNumber(1)
  set key($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearKey() => clearField(1);
}

class PreKey extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'PreKey', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<PublicKey>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'pubKey', subBuilder: PublicKey.create)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OU3)
    ..e<KeyScheme>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'scheme', $pb.PbFieldType.OE, defaultOrMaker: KeyScheme.KEY_SCHEME_ED25519, valueOf: KeyScheme.valueOf, enumValues: KeyScheme.values)
    ..hasRequiredFields = false
  ;

  PreKey._() : super();
  factory PreKey({
    PublicKey? pubKey,
    $core.int? id,
    KeyScheme? scheme,
  }) {
    final _result = create();
    if (pubKey != null) {
      _result.pubKey = pubKey;
    }
    if (id != null) {
      _result.id = id;
    }
    if (scheme != null) {
      _result.scheme = scheme;
    }
    return _result;
  }
  factory PreKey.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PreKey.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PreKey clone() => PreKey()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PreKey copyWith(void Function(PreKey) updates) => super.copyWith((message) => updates(message as PreKey)) as PreKey; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PreKey create() => PreKey._();
  PreKey createEmptyInstance() => create();
  static $pb.PbList<PreKey> createRepeated() => $pb.PbList<PreKey>();
  @$core.pragma('dart2js:noInline')
  static PreKey getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PreKey>(create);
  static PreKey? _defaultInstance;

  @$pb.TagNumber(1)
  PublicKey get pubKey => $_getN(0);
  @$pb.TagNumber(1)
  set pubKey(PublicKey v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasPubKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearPubKey() => clearField(1);
  @$pb.TagNumber(1)
  PublicKey ensurePubKey() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.int get id => $_getIZ(1);
  @$pb.TagNumber(2)
  set id($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasId() => $_has(1);
  @$pb.TagNumber(2)
  void clearId() => clearField(2);

  @$pb.TagNumber(3)
  KeyScheme get scheme => $_getN(2);
  @$pb.TagNumber(3)
  set scheme(KeyScheme v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasScheme() => $_has(2);
  @$pb.TagNumber(3)
  void clearScheme() => clearField(3);
}

class KeyPair extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'KeyPair', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<PrivateKey>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'privateKey', subBuilder: PrivateKey.create)
    ..aOM<PublicKey>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'publicKey', subBuilder: PublicKey.create)
    ..e<KeyScheme>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'scheme', $pb.PbFieldType.OE, defaultOrMaker: KeyScheme.KEY_SCHEME_ED25519, valueOf: KeyScheme.valueOf, enumValues: KeyScheme.values)
    ..hasRequiredFields = false
  ;

  KeyPair._() : super();
  factory KeyPair({
    PrivateKey? privateKey,
    PublicKey? publicKey,
    KeyScheme? scheme,
  }) {
    final _result = create();
    if (privateKey != null) {
      _result.privateKey = privateKey;
    }
    if (publicKey != null) {
      _result.publicKey = publicKey;
    }
    if (scheme != null) {
      _result.scheme = scheme;
    }
    return _result;
  }
  factory KeyPair.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory KeyPair.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  KeyPair clone() => KeyPair()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  KeyPair copyWith(void Function(KeyPair) updates) => super.copyWith((message) => updates(message as KeyPair)) as KeyPair; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static KeyPair create() => KeyPair._();
  KeyPair createEmptyInstance() => create();
  static $pb.PbList<KeyPair> createRepeated() => $pb.PbList<KeyPair>();
  @$core.pragma('dart2js:noInline')
  static KeyPair getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<KeyPair>(create);
  static KeyPair? _defaultInstance;

  @$pb.TagNumber(1)
  PrivateKey get privateKey => $_getN(0);
  @$pb.TagNumber(1)
  set privateKey(PrivateKey v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasPrivateKey() => $_has(0);
  @$pb.TagNumber(1)
  void clearPrivateKey() => clearField(1);
  @$pb.TagNumber(1)
  PrivateKey ensurePrivateKey() => $_ensure(0);

  @$pb.TagNumber(2)
  PublicKey get publicKey => $_getN(1);
  @$pb.TagNumber(2)
  set publicKey(PublicKey v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasPublicKey() => $_has(1);
  @$pb.TagNumber(2)
  void clearPublicKey() => clearField(2);
  @$pb.TagNumber(2)
  PublicKey ensurePublicKey() => $_ensure(1);

  @$pb.TagNumber(3)
  KeyScheme get scheme => $_getN(2);
  @$pb.TagNumber(3)
  set scheme(KeyScheme v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasScheme() => $_has(2);
  @$pb.TagNumber(3)
  void clearScheme() => clearField(3);
}

class Signature extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Signature', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..e<KeyScheme>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'scheme', $pb.PbFieldType.OE, defaultOrMaker: KeyScheme.KEY_SCHEME_ED25519, valueOf: KeyScheme.valueOf, enumValues: KeyScheme.values)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  Signature._() : super();
  factory Signature({
    KeyScheme? scheme,
    $core.List<$core.int>? signature,
  }) {
    final _result = create();
    if (scheme != null) {
      _result.scheme = scheme;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory Signature.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Signature.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Signature clone() => Signature()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Signature copyWith(void Function(Signature) updates) => super.copyWith((message) => updates(message as Signature)) as Signature; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Signature create() => Signature._();
  Signature createEmptyInstance() => create();
  static $pb.PbList<Signature> createRepeated() => $pb.PbList<Signature>();
  @$core.pragma('dart2js:noInline')
  static Signature getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Signature>(create);
  static Signature? _defaultInstance;

  @$pb.TagNumber(1)
  KeyScheme get scheme => $_getN(0);
  @$pb.TagNumber(1)
  set scheme(KeyScheme v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasScheme() => $_has(0);
  @$pb.TagNumber(1)
  void clearScheme() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => clearField(2);
}

class MobileNumber extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MobileNumber', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'number')
    ..hasRequiredFields = false
  ;

  MobileNumber._() : super();
  factory MobileNumber({
    $core.String? number,
  }) {
    final _result = create();
    if (number != null) {
      _result.number = number;
    }
    return _result;
  }
  factory MobileNumber.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MobileNumber.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MobileNumber clone() => MobileNumber()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MobileNumber copyWith(void Function(MobileNumber) updates) => super.copyWith((message) => updates(message as MobileNumber)) as MobileNumber; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MobileNumber create() => MobileNumber._();
  MobileNumber createEmptyInstance() => create();
  static $pb.PbList<MobileNumber> createRepeated() => $pb.PbList<MobileNumber>();
  @$core.pragma('dart2js:noInline')
  static MobileNumber getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MobileNumber>(create);
  static MobileNumber? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get number => $_getSZ(0);
  @$pb.TagNumber(1)
  set number($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNumber() => $_has(0);
  @$pb.TagNumber(1)
  void clearNumber() => clearField(1);
}

class User extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'User', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: AccountId.create)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nonce', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userName')
    ..aOM<MobileNumber>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: MobileNumber.create)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'balance', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..pc<TraitScore>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'traitScores', $pb.PbFieldType.PM, subBuilder: TraitScore.create)
    ..pc<PreKey>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'preKeys', $pb.PbFieldType.PM, subBuilder: PreKey.create)
    ..a<$core.int>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'karmaScore', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  User._() : super();
  factory User({
    AccountId? accountId,
    $fixnum.Int64? nonce,
    $core.String? userName,
    MobileNumber? mobileNumber,
    $fixnum.Int64? balance,
    $core.Iterable<TraitScore>? traitScores,
    $core.Iterable<PreKey>? preKeys,
    $core.int? karmaScore,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (nonce != null) {
      _result.nonce = nonce;
    }
    if (userName != null) {
      _result.userName = userName;
    }
    if (mobileNumber != null) {
      _result.mobileNumber = mobileNumber;
    }
    if (balance != null) {
      _result.balance = balance;
    }
    if (traitScores != null) {
      _result.traitScores.addAll(traitScores);
    }
    if (preKeys != null) {
      _result.preKeys.addAll(preKeys);
    }
    if (karmaScore != null) {
      _result.karmaScore = karmaScore;
    }
    return _result;
  }
  factory User.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory User.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  User clone() => User()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  User copyWith(void Function(User) updates) => super.copyWith((message) => updates(message as User)) as User; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static User create() => User._();
  User createEmptyInstance() => create();
  static $pb.PbList<User> createRepeated() => $pb.PbList<User>();
  @$core.pragma('dart2js:noInline')
  static User getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<User>(create);
  static User? _defaultInstance;

  @$pb.TagNumber(1)
  AccountId get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId(AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);
  @$pb.TagNumber(1)
  AccountId ensureAccountId() => $_ensure(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get nonce => $_getI64(1);
  @$pb.TagNumber(2)
  set nonce($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNonce() => $_has(1);
  @$pb.TagNumber(2)
  void clearNonce() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get userName => $_getSZ(2);
  @$pb.TagNumber(3)
  set userName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUserName() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserName() => clearField(3);

  @$pb.TagNumber(4)
  MobileNumber get mobileNumber => $_getN(3);
  @$pb.TagNumber(4)
  set mobileNumber(MobileNumber v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasMobileNumber() => $_has(3);
  @$pb.TagNumber(4)
  void clearMobileNumber() => clearField(4);
  @$pb.TagNumber(4)
  MobileNumber ensureMobileNumber() => $_ensure(3);

  @$pb.TagNumber(5)
  $fixnum.Int64 get balance => $_getI64(4);
  @$pb.TagNumber(5)
  set balance($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasBalance() => $_has(4);
  @$pb.TagNumber(5)
  void clearBalance() => clearField(5);

  @$pb.TagNumber(6)
  $core.List<TraitScore> get traitScores => $_getList(5);

  @$pb.TagNumber(7)
  $core.List<PreKey> get preKeys => $_getList(6);

  @$pb.TagNumber(8)
  $core.int get karmaScore => $_getIZ(7);
  @$pb.TagNumber(8)
  set karmaScore($core.int v) { $_setUnsignedInt32(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasKarmaScore() => $_has(7);
  @$pb.TagNumber(8)
  void clearKarmaScore() => clearField(8);
}

class PhoneVerifier extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'PhoneVerifier', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: AccountId.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..hasRequiredFields = false
  ;

  PhoneVerifier._() : super();
  factory PhoneVerifier({
    AccountId? accountId,
    $core.String? name,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (name != null) {
      _result.name = name;
    }
    return _result;
  }
  factory PhoneVerifier.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PhoneVerifier.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PhoneVerifier clone() => PhoneVerifier()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PhoneVerifier copyWith(void Function(PhoneVerifier) updates) => super.copyWith((message) => updates(message as PhoneVerifier)) as PhoneVerifier; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PhoneVerifier create() => PhoneVerifier._();
  PhoneVerifier createEmptyInstance() => create();
  static $pb.PbList<PhoneVerifier> createRepeated() => $pb.PbList<PhoneVerifier>();
  @$core.pragma('dart2js:noInline')
  static PhoneVerifier getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PhoneVerifier>(create);
  static PhoneVerifier? _defaultInstance;

  @$pb.TagNumber(1)
  AccountId get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId(AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);
  @$pb.TagNumber(1)
  AccountId ensureAccountId() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);
}

class Block extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Block', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'time', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<AccountId>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'author', subBuilder: AccountId.create)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'height', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..p<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionsHashes', $pb.PbFieldType.PY)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fees', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'prevBlockDigest', $pb.PbFieldType.OY)
    ..aOM<Signature>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', subBuilder: Signature.create)
    ..a<$fixnum.Int64>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'reward', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'minted', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'digest', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  Block._() : super();
  factory Block({
    $fixnum.Int64? time,
    AccountId? author,
    $fixnum.Int64? height,
    $core.Iterable<$core.List<$core.int>>? transactionsHashes,
    $fixnum.Int64? fees,
    $core.List<$core.int>? prevBlockDigest,
    Signature? signature,
    $fixnum.Int64? reward,
    $fixnum.Int64? minted,
    $core.List<$core.int>? digest,
  }) {
    final _result = create();
    if (time != null) {
      _result.time = time;
    }
    if (author != null) {
      _result.author = author;
    }
    if (height != null) {
      _result.height = height;
    }
    if (transactionsHashes != null) {
      _result.transactionsHashes.addAll(transactionsHashes);
    }
    if (fees != null) {
      _result.fees = fees;
    }
    if (prevBlockDigest != null) {
      _result.prevBlockDigest = prevBlockDigest;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    if (reward != null) {
      _result.reward = reward;
    }
    if (minted != null) {
      _result.minted = minted;
    }
    if (digest != null) {
      _result.digest = digest;
    }
    return _result;
  }
  factory Block.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Block.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Block clone() => Block()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Block copyWith(void Function(Block) updates) => super.copyWith((message) => updates(message as Block)) as Block; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Block create() => Block._();
  Block createEmptyInstance() => create();
  static $pb.PbList<Block> createRepeated() => $pb.PbList<Block>();
  @$core.pragma('dart2js:noInline')
  static Block getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Block>(create);
  static Block? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get time => $_getI64(0);
  @$pb.TagNumber(1)
  set time($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTime() => $_has(0);
  @$pb.TagNumber(1)
  void clearTime() => clearField(1);

  @$pb.TagNumber(2)
  AccountId get author => $_getN(1);
  @$pb.TagNumber(2)
  set author(AccountId v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasAuthor() => $_has(1);
  @$pb.TagNumber(2)
  void clearAuthor() => clearField(2);
  @$pb.TagNumber(2)
  AccountId ensureAuthor() => $_ensure(1);

  @$pb.TagNumber(3)
  $fixnum.Int64 get height => $_getI64(2);
  @$pb.TagNumber(3)
  set height($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasHeight() => $_has(2);
  @$pb.TagNumber(3)
  void clearHeight() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.List<$core.int>> get transactionsHashes => $_getList(3);

  @$pb.TagNumber(5)
  $fixnum.Int64 get fees => $_getI64(4);
  @$pb.TagNumber(5)
  set fees($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasFees() => $_has(4);
  @$pb.TagNumber(5)
  void clearFees() => clearField(5);

  @$pb.TagNumber(6)
  $core.List<$core.int> get prevBlockDigest => $_getN(5);
  @$pb.TagNumber(6)
  set prevBlockDigest($core.List<$core.int> v) { $_setBytes(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasPrevBlockDigest() => $_has(5);
  @$pb.TagNumber(6)
  void clearPrevBlockDigest() => clearField(6);

  @$pb.TagNumber(7)
  Signature get signature => $_getN(6);
  @$pb.TagNumber(7)
  set signature(Signature v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasSignature() => $_has(6);
  @$pb.TagNumber(7)
  void clearSignature() => clearField(7);
  @$pb.TagNumber(7)
  Signature ensureSignature() => $_ensure(6);

  @$pb.TagNumber(8)
  $fixnum.Int64 get reward => $_getI64(7);
  @$pb.TagNumber(8)
  set reward($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasReward() => $_has(7);
  @$pb.TagNumber(8)
  void clearReward() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get minted => $_getI64(8);
  @$pb.TagNumber(9)
  set minted($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasMinted() => $_has(8);
  @$pb.TagNumber(9)
  void clearMinted() => clearField(9);

  @$pb.TagNumber(10)
  $core.List<$core.int> get digest => $_getN(9);
  @$pb.TagNumber(10)
  set digest($core.List<$core.int> v) { $_setBytes(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasDigest() => $_has(9);
  @$pb.TagNumber(10)
  void clearDigest() => clearField(10);
}

class CharTrait extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CharTrait', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OU3)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'emoji')
    ..hasRequiredFields = false
  ;

  CharTrait._() : super();
  factory CharTrait({
    $core.int? id,
    $core.String? name,
    $core.String? emoji,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (name != null) {
      _result.name = name;
    }
    if (emoji != null) {
      _result.emoji = emoji;
    }
    return _result;
  }
  factory CharTrait.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CharTrait.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CharTrait clone() => CharTrait()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CharTrait copyWith(void Function(CharTrait) updates) => super.copyWith((message) => updates(message as CharTrait)) as CharTrait; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CharTrait create() => CharTrait._();
  CharTrait createEmptyInstance() => create();
  static $pb.PbList<CharTrait> createRepeated() => $pb.PbList<CharTrait>();
  @$core.pragma('dart2js:noInline')
  static CharTrait getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CharTrait>(create);
  static CharTrait? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get emoji => $_getSZ(2);
  @$pb.TagNumber(3)
  set emoji($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasEmoji() => $_has(2);
  @$pb.TagNumber(3)
  void clearEmoji() => clearField(3);
}

class TraitScore extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TraitScore', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'traitId', $pb.PbFieldType.OU3)
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'score', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  TraitScore._() : super();
  factory TraitScore({
    $core.int? traitId,
    $core.int? score,
  }) {
    final _result = create();
    if (traitId != null) {
      _result.traitId = traitId;
    }
    if (score != null) {
      _result.score = score;
    }
    return _result;
  }
  factory TraitScore.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TraitScore.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TraitScore clone() => TraitScore()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TraitScore copyWith(void Function(TraitScore) updates) => super.copyWith((message) => updates(message as TraitScore)) as TraitScore; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TraitScore create() => TraitScore._();
  TraitScore createEmptyInstance() => create();
  static $pb.PbList<TraitScore> createRepeated() => $pb.PbList<TraitScore>();
  @$core.pragma('dart2js:noInline')
  static TraitScore getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TraitScore>(create);
  static TraitScore? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get traitId => $_getIZ(0);
  @$pb.TagNumber(1)
  set traitId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTraitId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTraitId() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get score => $_getIZ(1);
  @$pb.TagNumber(2)
  set score($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasScore() => $_has(1);
  @$pb.TagNumber(2)
  void clearScore() => clearField(2);
}

class NewUserTransactionV1 extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NewUserTransactionV1', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<UserVerificationData>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verifyNumberResponse', subBuilder: UserVerificationData.create)
    ..hasRequiredFields = false
  ;

  NewUserTransactionV1._() : super();
  factory NewUserTransactionV1({
    UserVerificationData? verifyNumberResponse,
  }) {
    final _result = create();
    if (verifyNumberResponse != null) {
      _result.verifyNumberResponse = verifyNumberResponse;
    }
    return _result;
  }
  factory NewUserTransactionV1.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewUserTransactionV1.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewUserTransactionV1 clone() => NewUserTransactionV1()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewUserTransactionV1 copyWith(void Function(NewUserTransactionV1) updates) => super.copyWith((message) => updates(message as NewUserTransactionV1)) as NewUserTransactionV1; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NewUserTransactionV1 create() => NewUserTransactionV1._();
  NewUserTransactionV1 createEmptyInstance() => create();
  static $pb.PbList<NewUserTransactionV1> createRepeated() => $pb.PbList<NewUserTransactionV1>();
  @$core.pragma('dart2js:noInline')
  static NewUserTransactionV1 getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewUserTransactionV1>(create);
  static NewUserTransactionV1? _defaultInstance;

  @$pb.TagNumber(1)
  UserVerificationData get verifyNumberResponse => $_getN(0);
  @$pb.TagNumber(1)
  set verifyNumberResponse(UserVerificationData v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasVerifyNumberResponse() => $_has(0);
  @$pb.TagNumber(1)
  void clearVerifyNumberResponse() => clearField(1);
  @$pb.TagNumber(1)
  UserVerificationData ensureVerifyNumberResponse() => $_ensure(0);
}

class PaymentTransactionV1 extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'PaymentTransactionV1', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'from', subBuilder: AccountId.create)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<MobileNumber>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'to', subBuilder: MobileNumber.create)
    ..a<$core.int>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'charTraitId', $pb.PbFieldType.OU3)
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'communityId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  PaymentTransactionV1._() : super();
  factory PaymentTransactionV1({
    AccountId? from,
    $fixnum.Int64? amount,
    MobileNumber? to,
    $core.int? charTraitId,
    $core.int? communityId,
  }) {
    final _result = create();
    if (from != null) {
      _result.from = from;
    }
    if (amount != null) {
      _result.amount = amount;
    }
    if (to != null) {
      _result.to = to;
    }
    if (charTraitId != null) {
      _result.charTraitId = charTraitId;
    }
    if (communityId != null) {
      _result.communityId = communityId;
    }
    return _result;
  }
  factory PaymentTransactionV1.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PaymentTransactionV1.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PaymentTransactionV1 clone() => PaymentTransactionV1()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PaymentTransactionV1 copyWith(void Function(PaymentTransactionV1) updates) => super.copyWith((message) => updates(message as PaymentTransactionV1)) as PaymentTransactionV1; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PaymentTransactionV1 create() => PaymentTransactionV1._();
  PaymentTransactionV1 createEmptyInstance() => create();
  static $pb.PbList<PaymentTransactionV1> createRepeated() => $pb.PbList<PaymentTransactionV1>();
  @$core.pragma('dart2js:noInline')
  static PaymentTransactionV1 getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PaymentTransactionV1>(create);
  static PaymentTransactionV1? _defaultInstance;

  @$pb.TagNumber(1)
  AccountId get from => $_getN(0);
  @$pb.TagNumber(1)
  set from(AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearFrom() => clearField(1);
  @$pb.TagNumber(1)
  AccountId ensureFrom() => $_ensure(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get amount => $_getI64(1);
  @$pb.TagNumber(2)
  set amount($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAmount() => $_has(1);
  @$pb.TagNumber(2)
  void clearAmount() => clearField(2);

  @$pb.TagNumber(3)
  MobileNumber get to => $_getN(2);
  @$pb.TagNumber(3)
  set to(MobileNumber v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasTo() => $_has(2);
  @$pb.TagNumber(3)
  void clearTo() => clearField(3);
  @$pb.TagNumber(3)
  MobileNumber ensureTo() => $_ensure(2);

  @$pb.TagNumber(4)
  $core.int get charTraitId => $_getIZ(3);
  @$pb.TagNumber(4)
  set charTraitId($core.int v) { $_setUnsignedInt32(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasCharTraitId() => $_has(3);
  @$pb.TagNumber(4)
  void clearCharTraitId() => clearField(4);

  @$pb.TagNumber(5)
  $core.int get communityId => $_getIZ(4);
  @$pb.TagNumber(5)
  set communityId($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasCommunityId() => $_has(4);
  @$pb.TagNumber(5)
  void clearCommunityId() => clearField(5);
}

class UpdateUserTransactionV1 extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'UpdateUserTransactionV1', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nickname')
    ..aOM<MobileNumber>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: MobileNumber.create)
    ..aOM<UserVerificationData>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userVerificationData', subBuilder: UserVerificationData.create)
    ..hasRequiredFields = false
  ;

  UpdateUserTransactionV1._() : super();
  factory UpdateUserTransactionV1({
    $core.String? nickname,
    MobileNumber? mobileNumber,
    UserVerificationData? userVerificationData,
  }) {
    final _result = create();
    if (nickname != null) {
      _result.nickname = nickname;
    }
    if (mobileNumber != null) {
      _result.mobileNumber = mobileNumber;
    }
    if (userVerificationData != null) {
      _result.userVerificationData = userVerificationData;
    }
    return _result;
  }
  factory UpdateUserTransactionV1.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UpdateUserTransactionV1.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UpdateUserTransactionV1 clone() => UpdateUserTransactionV1()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UpdateUserTransactionV1 copyWith(void Function(UpdateUserTransactionV1) updates) => super.copyWith((message) => updates(message as UpdateUserTransactionV1)) as UpdateUserTransactionV1; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UpdateUserTransactionV1 create() => UpdateUserTransactionV1._();
  UpdateUserTransactionV1 createEmptyInstance() => create();
  static $pb.PbList<UpdateUserTransactionV1> createRepeated() => $pb.PbList<UpdateUserTransactionV1>();
  @$core.pragma('dart2js:noInline')
  static UpdateUserTransactionV1 getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UpdateUserTransactionV1>(create);
  static UpdateUserTransactionV1? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get nickname => $_getSZ(0);
  @$pb.TagNumber(1)
  set nickname($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNickname() => $_has(0);
  @$pb.TagNumber(1)
  void clearNickname() => clearField(1);

  @$pb.TagNumber(2)
  MobileNumber get mobileNumber => $_getN(1);
  @$pb.TagNumber(2)
  set mobileNumber(MobileNumber v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasMobileNumber() => $_has(1);
  @$pb.TagNumber(2)
  void clearMobileNumber() => clearField(2);
  @$pb.TagNumber(2)
  MobileNumber ensureMobileNumber() => $_ensure(1);

  @$pb.TagNumber(3)
  UserVerificationData get userVerificationData => $_getN(2);
  @$pb.TagNumber(3)
  set userVerificationData(UserVerificationData v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasUserVerificationData() => $_has(2);
  @$pb.TagNumber(3)
  void clearUserVerificationData() => clearField(3);
  @$pb.TagNumber(3)
  UserVerificationData ensureUserVerificationData() => $_ensure(2);
}

class TransactionBody extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionBody', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nonce', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fee', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<TransactionData>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionData', subBuilder: TransactionData.create)
    ..a<$core.int>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'netId', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  TransactionBody._() : super();
  factory TransactionBody({
    $fixnum.Int64? timestamp,
    $fixnum.Int64? nonce,
    $fixnum.Int64? fee,
    TransactionData? transactionData,
    $core.int? netId,
  }) {
    final _result = create();
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (nonce != null) {
      _result.nonce = nonce;
    }
    if (fee != null) {
      _result.fee = fee;
    }
    if (transactionData != null) {
      _result.transactionData = transactionData;
    }
    if (netId != null) {
      _result.netId = netId;
    }
    return _result;
  }
  factory TransactionBody.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionBody.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionBody clone() => TransactionBody()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionBody copyWith(void Function(TransactionBody) updates) => super.copyWith((message) => updates(message as TransactionBody)) as TransactionBody; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TransactionBody create() => TransactionBody._();
  TransactionBody createEmptyInstance() => create();
  static $pb.PbList<TransactionBody> createRepeated() => $pb.PbList<TransactionBody>();
  @$core.pragma('dart2js:noInline')
  static TransactionBody getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionBody>(create);
  static TransactionBody? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get timestamp => $_getI64(0);
  @$pb.TagNumber(1)
  set timestamp($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get nonce => $_getI64(1);
  @$pb.TagNumber(2)
  set nonce($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNonce() => $_has(1);
  @$pb.TagNumber(2)
  void clearNonce() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get fee => $_getI64(2);
  @$pb.TagNumber(3)
  set fee($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFee() => $_has(2);
  @$pb.TagNumber(3)
  void clearFee() => clearField(3);

  @$pb.TagNumber(4)
  TransactionData get transactionData => $_getN(3);
  @$pb.TagNumber(4)
  set transactionData(TransactionData v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasTransactionData() => $_has(3);
  @$pb.TagNumber(4)
  void clearTransactionData() => clearField(4);
  @$pb.TagNumber(4)
  TransactionData ensureTransactionData() => $_ensure(3);

  @$pb.TagNumber(5)
  $core.int get netId => $_getIZ(4);
  @$pb.TagNumber(5)
  set netId($core.int v) { $_setUnsignedInt32(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasNetId() => $_has(4);
  @$pb.TagNumber(5)
  void clearNetId() => clearField(5);
}

class TransactionData extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionData', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionData', $pb.PbFieldType.OY)
    ..e<TransactionType>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionType', $pb.PbFieldType.OE, defaultOrMaker: TransactionType.TRANSACTION_TYPE_PAYMENT_V1, valueOf: TransactionType.valueOf, enumValues: TransactionType.values)
    ..hasRequiredFields = false
  ;

  TransactionData._() : super();
  factory TransactionData({
    $core.List<$core.int>? transactionData,
    TransactionType? transactionType,
  }) {
    final _result = create();
    if (transactionData != null) {
      _result.transactionData = transactionData;
    }
    if (transactionType != null) {
      _result.transactionType = transactionType;
    }
    return _result;
  }
  factory TransactionData.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionData.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionData clone() => TransactionData()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionData copyWith(void Function(TransactionData) updates) => super.copyWith((message) => updates(message as TransactionData)) as TransactionData; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TransactionData create() => TransactionData._();
  TransactionData createEmptyInstance() => create();
  static $pb.PbList<TransactionData> createRepeated() => $pb.PbList<TransactionData>();
  @$core.pragma('dart2js:noInline')
  static TransactionData getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionData>(create);
  static TransactionData? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get transactionData => $_getN(0);
  @$pb.TagNumber(1)
  set transactionData($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransactionData() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransactionData() => clearField(1);

  @$pb.TagNumber(2)
  TransactionType get transactionType => $_getN(1);
  @$pb.TagNumber(2)
  set transactionType(TransactionType v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTransactionType() => $_has(1);
  @$pb.TagNumber(2)
  void clearTransactionType() => clearField(2);
}

class SignedTransaction extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SignedTransaction', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signer', subBuilder: AccountId.create)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionBody', $pb.PbFieldType.OY)
    ..aOM<Signature>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', subBuilder: Signature.create)
    ..hasRequiredFields = false
  ;

  SignedTransaction._() : super();
  factory SignedTransaction({
    AccountId? signer,
    $core.List<$core.int>? transactionBody,
    Signature? signature,
  }) {
    final _result = create();
    if (signer != null) {
      _result.signer = signer;
    }
    if (transactionBody != null) {
      _result.transactionBody = transactionBody;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory SignedTransaction.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SignedTransaction.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SignedTransaction clone() => SignedTransaction()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SignedTransaction copyWith(void Function(SignedTransaction) updates) => super.copyWith((message) => updates(message as SignedTransaction)) as SignedTransaction; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SignedTransaction create() => SignedTransaction._();
  SignedTransaction createEmptyInstance() => create();
  static $pb.PbList<SignedTransaction> createRepeated() => $pb.PbList<SignedTransaction>();
  @$core.pragma('dart2js:noInline')
  static SignedTransaction getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SignedTransaction>(create);
  static SignedTransaction? _defaultInstance;

  @$pb.TagNumber(1)
  AccountId get signer => $_getN(0);
  @$pb.TagNumber(1)
  set signer(AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasSigner() => $_has(0);
  @$pb.TagNumber(1)
  void clearSigner() => clearField(1);
  @$pb.TagNumber(1)
  AccountId ensureSigner() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<$core.int> get transactionBody => $_getN(1);
  @$pb.TagNumber(2)
  set transactionBody($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTransactionBody() => $_has(1);
  @$pb.TagNumber(2)
  void clearTransactionBody() => clearField(2);

  @$pb.TagNumber(3)
  Signature get signature => $_getN(2);
  @$pb.TagNumber(3)
  set signature(Signature v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasSignature() => $_has(2);
  @$pb.TagNumber(3)
  void clearSignature() => clearField(3);
  @$pb.TagNumber(3)
  Signature ensureSignature() => $_ensure(2);
}

class UserVerificationData extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'UserVerificationData', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<AccountId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verifierAccountId', subBuilder: AccountId.create)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<VerificationResult>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'verificationResult', $pb.PbFieldType.OE, defaultOrMaker: VerificationResult.VERIFICATION_RESULT_UNSPECIFIED, valueOf: VerificationResult.valueOf, enumValues: VerificationResult.values)
    ..aOM<AccountId>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', subBuilder: AccountId.create)
    ..aOM<MobileNumber>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mobileNumber', subBuilder: MobileNumber.create)
    ..aOS(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'requestedUserName')
    ..aOM<Signature>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', subBuilder: Signature.create)
    ..hasRequiredFields = false
  ;

  UserVerificationData._() : super();
  factory UserVerificationData({
    AccountId? verifierAccountId,
    $fixnum.Int64? timestamp,
    VerificationResult? verificationResult,
    AccountId? accountId,
    MobileNumber? mobileNumber,
    $core.String? requestedUserName,
    Signature? signature,
  }) {
    final _result = create();
    if (verifierAccountId != null) {
      _result.verifierAccountId = verifierAccountId;
    }
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (verificationResult != null) {
      _result.verificationResult = verificationResult;
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
  factory UserVerificationData.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory UserVerificationData.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  UserVerificationData clone() => UserVerificationData()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  UserVerificationData copyWith(void Function(UserVerificationData) updates) => super.copyWith((message) => updates(message as UserVerificationData)) as UserVerificationData; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static UserVerificationData create() => UserVerificationData._();
  UserVerificationData createEmptyInstance() => create();
  static $pb.PbList<UserVerificationData> createRepeated() => $pb.PbList<UserVerificationData>();
  @$core.pragma('dart2js:noInline')
  static UserVerificationData getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<UserVerificationData>(create);
  static UserVerificationData? _defaultInstance;

  @$pb.TagNumber(1)
  AccountId get verifierAccountId => $_getN(0);
  @$pb.TagNumber(1)
  set verifierAccountId(AccountId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasVerifierAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearVerifierAccountId() => clearField(1);
  @$pb.TagNumber(1)
  AccountId ensureVerifierAccountId() => $_ensure(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get timestamp => $_getI64(1);
  @$pb.TagNumber(2)
  set timestamp($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTimestamp() => $_has(1);
  @$pb.TagNumber(2)
  void clearTimestamp() => clearField(2);

  @$pb.TagNumber(3)
  VerificationResult get verificationResult => $_getN(2);
  @$pb.TagNumber(3)
  set verificationResult(VerificationResult v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasVerificationResult() => $_has(2);
  @$pb.TagNumber(3)
  void clearVerificationResult() => clearField(3);

  @$pb.TagNumber(4)
  AccountId get accountId => $_getN(3);
  @$pb.TagNumber(4)
  set accountId(AccountId v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasAccountId() => $_has(3);
  @$pb.TagNumber(4)
  void clearAccountId() => clearField(4);
  @$pb.TagNumber(4)
  AccountId ensureAccountId() => $_ensure(3);

  @$pb.TagNumber(5)
  MobileNumber get mobileNumber => $_getN(4);
  @$pb.TagNumber(5)
  set mobileNumber(MobileNumber v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasMobileNumber() => $_has(4);
  @$pb.TagNumber(5)
  void clearMobileNumber() => clearField(5);
  @$pb.TagNumber(5)
  MobileNumber ensureMobileNumber() => $_ensure(4);

  @$pb.TagNumber(7)
  $core.String get requestedUserName => $_getSZ(5);
  @$pb.TagNumber(7)
  set requestedUserName($core.String v) { $_setString(5, v); }
  @$pb.TagNumber(7)
  $core.bool hasRequestedUserName() => $_has(5);
  @$pb.TagNumber(7)
  void clearRequestedUserName() => clearField(7);

  @$pb.TagNumber(8)
  Signature get signature => $_getN(6);
  @$pb.TagNumber(8)
  set signature(Signature v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasSignature() => $_has(6);
  @$pb.TagNumber(8)
  void clearSignature() => clearField(8);
  @$pb.TagNumber(8)
  Signature ensureSignature() => $_ensure(6);
}

class SignedTransactionsHashes extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SignedTransactionsHashes', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..p<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'hashes', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  SignedTransactionsHashes._() : super();
  factory SignedTransactionsHashes({
    $core.Iterable<$core.List<$core.int>>? hashes,
  }) {
    final _result = create();
    if (hashes != null) {
      _result.hashes.addAll(hashes);
    }
    return _result;
  }
  factory SignedTransactionsHashes.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SignedTransactionsHashes.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SignedTransactionsHashes clone() => SignedTransactionsHashes()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SignedTransactionsHashes copyWith(void Function(SignedTransactionsHashes) updates) => super.copyWith((message) => updates(message as SignedTransactionsHashes)) as SignedTransactionsHashes; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SignedTransactionsHashes create() => SignedTransactionsHashes._();
  SignedTransactionsHashes createEmptyInstance() => create();
  static $pb.PbList<SignedTransactionsHashes> createRepeated() => $pb.PbList<SignedTransactionsHashes>();
  @$core.pragma('dart2js:noInline')
  static SignedTransactionsHashes getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SignedTransactionsHashes>(create);
  static SignedTransactionsHashes? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.List<$core.int>> get hashes => $_getList(0);
}

class MemPool extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'MemPool', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..pc<SignedTransaction>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactions', $pb.PbFieldType.PM, subBuilder: SignedTransaction.create)
    ..hasRequiredFields = false
  ;

  MemPool._() : super();
  factory MemPool({
    $core.Iterable<SignedTransaction>? transactions,
  }) {
    final _result = create();
    if (transactions != null) {
      _result.transactions.addAll(transactions);
    }
    return _result;
  }
  factory MemPool.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory MemPool.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  MemPool clone() => MemPool()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  MemPool copyWith(void Function(MemPool) updates) => super.copyWith((message) => updates(message as MemPool)) as MemPool; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static MemPool create() => MemPool._();
  MemPool createEmptyInstance() => create();
  static $pb.PbList<MemPool> createRepeated() => $pb.PbList<MemPool>();
  @$core.pragma('dart2js:noInline')
  static MemPool getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<MemPool>(create);
  static MemPool? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<SignedTransaction> get transactions => $_getList(0);
}

class SignedTransactionWithStatus extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SignedTransactionWithStatus', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..aOM<SignedTransaction>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transaction', subBuilder: SignedTransaction.create)
    ..e<TransactionStatus>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status', $pb.PbFieldType.OE, defaultOrMaker: TransactionStatus.TRANSACTION_STATUS_UNKNOWN, valueOf: TransactionStatus.valueOf, enumValues: TransactionStatus.values)
    ..hasRequiredFields = false
  ;

  SignedTransactionWithStatus._() : super();
  factory SignedTransactionWithStatus({
    SignedTransaction? transaction,
    TransactionStatus? status,
  }) {
    final _result = create();
    if (transaction != null) {
      _result.transaction = transaction;
    }
    if (status != null) {
      _result.status = status;
    }
    return _result;
  }
  factory SignedTransactionWithStatus.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SignedTransactionWithStatus.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SignedTransactionWithStatus clone() => SignedTransactionWithStatus()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SignedTransactionWithStatus copyWith(void Function(SignedTransactionWithStatus) updates) => super.copyWith((message) => updates(message as SignedTransactionWithStatus)) as SignedTransactionWithStatus; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SignedTransactionWithStatus create() => SignedTransactionWithStatus._();
  SignedTransactionWithStatus createEmptyInstance() => create();
  static $pb.PbList<SignedTransactionWithStatus> createRepeated() => $pb.PbList<SignedTransactionWithStatus>();
  @$core.pragma('dart2js:noInline')
  static SignedTransactionWithStatus getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SignedTransactionWithStatus>(create);
  static SignedTransactionWithStatus? _defaultInstance;

  @$pb.TagNumber(1)
  SignedTransaction get transaction => $_getN(0);
  @$pb.TagNumber(1)
  set transaction(SignedTransaction v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransaction() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransaction() => clearField(1);
  @$pb.TagNumber(1)
  SignedTransaction ensureTransaction() => $_ensure(0);

  @$pb.TagNumber(2)
  TransactionStatus get status => $_getN(1);
  @$pb.TagNumber(2)
  set status(TransactionStatus v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => clearField(2);
}

class TransactionEvent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionEvent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'height', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<SignedTransaction>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transaction', subBuilder: SignedTransaction.create)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionHash', $pb.PbFieldType.OY)
    ..e<ExecutionResult>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'result', $pb.PbFieldType.OE, defaultOrMaker: ExecutionResult.EXECUTION_RESULT_EXECUTED, valueOf: ExecutionResult.valueOf, enumValues: ExecutionResult.values)
    ..e<ExecutionInfo>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'info', $pb.PbFieldType.OE, defaultOrMaker: ExecutionInfo.EXECUTION_INFO_UNKNOWN, valueOf: ExecutionInfo.valueOf, enumValues: ExecutionInfo.values)
    ..aOS(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'errorMessage')
    ..e<FeeType>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feeType', $pb.PbFieldType.OE, defaultOrMaker: FeeType.FEE_TYPE_MINT, valueOf: FeeType.valueOf, enumValues: FeeType.values)
    ..a<$fixnum.Int64>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupReward', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralReward', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.int>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'appreciationCharTraitIdx', $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fee', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  TransactionEvent._() : super();
  factory TransactionEvent({
    $fixnum.Int64? timestamp,
    $fixnum.Int64? height,
    SignedTransaction? transaction,
    $core.List<$core.int>? transactionHash,
    ExecutionResult? result,
    ExecutionInfo? info,
    $core.String? errorMessage,
    FeeType? feeType,
    $fixnum.Int64? signupReward,
    $fixnum.Int64? referralReward,
    $core.int? appreciationCharTraitIdx,
    $fixnum.Int64? fee,
  }) {
    final _result = create();
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (height != null) {
      _result.height = height;
    }
    if (transaction != null) {
      _result.transaction = transaction;
    }
    if (transactionHash != null) {
      _result.transactionHash = transactionHash;
    }
    if (result != null) {
      _result.result = result;
    }
    if (info != null) {
      _result.info = info;
    }
    if (errorMessage != null) {
      _result.errorMessage = errorMessage;
    }
    if (feeType != null) {
      _result.feeType = feeType;
    }
    if (signupReward != null) {
      _result.signupReward = signupReward;
    }
    if (referralReward != null) {
      _result.referralReward = referralReward;
    }
    if (appreciationCharTraitIdx != null) {
      _result.appreciationCharTraitIdx = appreciationCharTraitIdx;
    }
    if (fee != null) {
      _result.fee = fee;
    }
    return _result;
  }
  factory TransactionEvent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionEvent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionEvent clone() => TransactionEvent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionEvent copyWith(void Function(TransactionEvent) updates) => super.copyWith((message) => updates(message as TransactionEvent)) as TransactionEvent; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TransactionEvent create() => TransactionEvent._();
  TransactionEvent createEmptyInstance() => create();
  static $pb.PbList<TransactionEvent> createRepeated() => $pb.PbList<TransactionEvent>();
  @$core.pragma('dart2js:noInline')
  static TransactionEvent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionEvent>(create);
  static TransactionEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get timestamp => $_getI64(0);
  @$pb.TagNumber(1)
  set timestamp($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get height => $_getI64(1);
  @$pb.TagNumber(2)
  set height($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearHeight() => clearField(2);

  @$pb.TagNumber(3)
  SignedTransaction get transaction => $_getN(2);
  @$pb.TagNumber(3)
  set transaction(SignedTransaction v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasTransaction() => $_has(2);
  @$pb.TagNumber(3)
  void clearTransaction() => clearField(3);
  @$pb.TagNumber(3)
  SignedTransaction ensureTransaction() => $_ensure(2);

  @$pb.TagNumber(4)
  $core.List<$core.int> get transactionHash => $_getN(3);
  @$pb.TagNumber(4)
  set transactionHash($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTransactionHash() => $_has(3);
  @$pb.TagNumber(4)
  void clearTransactionHash() => clearField(4);

  @$pb.TagNumber(5)
  ExecutionResult get result => $_getN(4);
  @$pb.TagNumber(5)
  set result(ExecutionResult v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasResult() => $_has(4);
  @$pb.TagNumber(5)
  void clearResult() => clearField(5);

  @$pb.TagNumber(6)
  ExecutionInfo get info => $_getN(5);
  @$pb.TagNumber(6)
  set info(ExecutionInfo v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasInfo() => $_has(5);
  @$pb.TagNumber(6)
  void clearInfo() => clearField(6);

  @$pb.TagNumber(7)
  $core.String get errorMessage => $_getSZ(6);
  @$pb.TagNumber(7)
  set errorMessage($core.String v) { $_setString(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasErrorMessage() => $_has(6);
  @$pb.TagNumber(7)
  void clearErrorMessage() => clearField(7);

  @$pb.TagNumber(8)
  FeeType get feeType => $_getN(7);
  @$pb.TagNumber(8)
  set feeType(FeeType v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasFeeType() => $_has(7);
  @$pb.TagNumber(8)
  void clearFeeType() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get signupReward => $_getI64(8);
  @$pb.TagNumber(9)
  set signupReward($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasSignupReward() => $_has(8);
  @$pb.TagNumber(9)
  void clearSignupReward() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get referralReward => $_getI64(9);
  @$pb.TagNumber(10)
  set referralReward($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasReferralReward() => $_has(9);
  @$pb.TagNumber(10)
  void clearReferralReward() => clearField(10);

  @$pb.TagNumber(11)
  $core.int get appreciationCharTraitIdx => $_getIZ(10);
  @$pb.TagNumber(11)
  set appreciationCharTraitIdx($core.int v) { $_setUnsignedInt32(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasAppreciationCharTraitIdx() => $_has(10);
  @$pb.TagNumber(11)
  void clearAppreciationCharTraitIdx() => clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get fee => $_getI64(11);
  @$pb.TagNumber(12)
  set fee($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasFee() => $_has(11);
  @$pb.TagNumber(12)
  void clearFee() => clearField(12);
}

class TransactionEvents extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionEvents', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..pc<TransactionEvent>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'events', $pb.PbFieldType.PM, subBuilder: TransactionEvent.create)
    ..hasRequiredFields = false
  ;

  TransactionEvents._() : super();
  factory TransactionEvents({
    $core.Iterable<TransactionEvent>? events,
  }) {
    final _result = create();
    if (events != null) {
      _result.events.addAll(events);
    }
    return _result;
  }
  factory TransactionEvents.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionEvents.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionEvents clone() => TransactionEvents()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionEvents copyWith(void Function(TransactionEvents) updates) => super.copyWith((message) => updates(message as TransactionEvents)) as TransactionEvents; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TransactionEvents create() => TransactionEvents._();
  TransactionEvents createEmptyInstance() => create();
  static $pb.PbList<TransactionEvents> createRepeated() => $pb.PbList<TransactionEvents>();
  @$core.pragma('dart2js:noInline')
  static TransactionEvents getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionEvents>(create);
  static TransactionEvents? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<TransactionEvent> get events => $_getList(0);
}

class BlockchainStats extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BlockchainStats', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastBlockTime', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'tipHeight', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'paymentsTransactionsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'appreciationsTransactionsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'usersCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feesAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mintedAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'circulation', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feeSubsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feeSubsAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(13, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardsAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(14, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(15, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardsAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(16, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'validatorRewardsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(17, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'validatorRewardsAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(18, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'updateUserTransactionsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.double>(19, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'exchangeRate', $pb.PbFieldType.OD)
    ..a<$fixnum.Int64>(20, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'causesRewardsAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  BlockchainStats._() : super();
  factory BlockchainStats({
    $fixnum.Int64? lastBlockTime,
    $fixnum.Int64? tipHeight,
    $fixnum.Int64? transactionsCount,
    $fixnum.Int64? paymentsTransactionsCount,
    $fixnum.Int64? appreciationsTransactionsCount,
    $fixnum.Int64? usersCount,
    $fixnum.Int64? feesAmount,
    $fixnum.Int64? mintedAmount,
    $fixnum.Int64? circulation,
    $fixnum.Int64? feeSubsCount,
    $fixnum.Int64? feeSubsAmount,
    $fixnum.Int64? signupRewardsCount,
    $fixnum.Int64? signupRewardsAmount,
    $fixnum.Int64? referralRewardsCount,
    $fixnum.Int64? referralRewardsAmount,
    $fixnum.Int64? validatorRewardsCount,
    $fixnum.Int64? validatorRewardsAmount,
    $fixnum.Int64? updateUserTransactionsCount,
    $core.double? exchangeRate,
    $fixnum.Int64? causesRewardsAmount,
  }) {
    final _result = create();
    if (lastBlockTime != null) {
      _result.lastBlockTime = lastBlockTime;
    }
    if (tipHeight != null) {
      _result.tipHeight = tipHeight;
    }
    if (transactionsCount != null) {
      _result.transactionsCount = transactionsCount;
    }
    if (paymentsTransactionsCount != null) {
      _result.paymentsTransactionsCount = paymentsTransactionsCount;
    }
    if (appreciationsTransactionsCount != null) {
      _result.appreciationsTransactionsCount = appreciationsTransactionsCount;
    }
    if (usersCount != null) {
      _result.usersCount = usersCount;
    }
    if (feesAmount != null) {
      _result.feesAmount = feesAmount;
    }
    if (mintedAmount != null) {
      _result.mintedAmount = mintedAmount;
    }
    if (circulation != null) {
      _result.circulation = circulation;
    }
    if (feeSubsCount != null) {
      _result.feeSubsCount = feeSubsCount;
    }
    if (feeSubsAmount != null) {
      _result.feeSubsAmount = feeSubsAmount;
    }
    if (signupRewardsCount != null) {
      _result.signupRewardsCount = signupRewardsCount;
    }
    if (signupRewardsAmount != null) {
      _result.signupRewardsAmount = signupRewardsAmount;
    }
    if (referralRewardsCount != null) {
      _result.referralRewardsCount = referralRewardsCount;
    }
    if (referralRewardsAmount != null) {
      _result.referralRewardsAmount = referralRewardsAmount;
    }
    if (validatorRewardsCount != null) {
      _result.validatorRewardsCount = validatorRewardsCount;
    }
    if (validatorRewardsAmount != null) {
      _result.validatorRewardsAmount = validatorRewardsAmount;
    }
    if (updateUserTransactionsCount != null) {
      _result.updateUserTransactionsCount = updateUserTransactionsCount;
    }
    if (exchangeRate != null) {
      _result.exchangeRate = exchangeRate;
    }
    if (causesRewardsAmount != null) {
      _result.causesRewardsAmount = causesRewardsAmount;
    }
    return _result;
  }
  factory BlockchainStats.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BlockchainStats.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BlockchainStats clone() => BlockchainStats()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BlockchainStats copyWith(void Function(BlockchainStats) updates) => super.copyWith((message) => updates(message as BlockchainStats)) as BlockchainStats; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BlockchainStats create() => BlockchainStats._();
  BlockchainStats createEmptyInstance() => create();
  static $pb.PbList<BlockchainStats> createRepeated() => $pb.PbList<BlockchainStats>();
  @$core.pragma('dart2js:noInline')
  static BlockchainStats getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BlockchainStats>(create);
  static BlockchainStats? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get lastBlockTime => $_getI64(0);
  @$pb.TagNumber(1)
  set lastBlockTime($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLastBlockTime() => $_has(0);
  @$pb.TagNumber(1)
  void clearLastBlockTime() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get tipHeight => $_getI64(1);
  @$pb.TagNumber(2)
  set tipHeight($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTipHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearTipHeight() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get transactionsCount => $_getI64(2);
  @$pb.TagNumber(3)
  set transactionsCount($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTransactionsCount() => $_has(2);
  @$pb.TagNumber(3)
  void clearTransactionsCount() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get paymentsTransactionsCount => $_getI64(3);
  @$pb.TagNumber(4)
  set paymentsTransactionsCount($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasPaymentsTransactionsCount() => $_has(3);
  @$pb.TagNumber(4)
  void clearPaymentsTransactionsCount() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get appreciationsTransactionsCount => $_getI64(4);
  @$pb.TagNumber(5)
  set appreciationsTransactionsCount($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasAppreciationsTransactionsCount() => $_has(4);
  @$pb.TagNumber(5)
  void clearAppreciationsTransactionsCount() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get usersCount => $_getI64(5);
  @$pb.TagNumber(6)
  set usersCount($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasUsersCount() => $_has(5);
  @$pb.TagNumber(6)
  void clearUsersCount() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get feesAmount => $_getI64(6);
  @$pb.TagNumber(7)
  set feesAmount($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasFeesAmount() => $_has(6);
  @$pb.TagNumber(7)
  void clearFeesAmount() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get mintedAmount => $_getI64(7);
  @$pb.TagNumber(8)
  set mintedAmount($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasMintedAmount() => $_has(7);
  @$pb.TagNumber(8)
  void clearMintedAmount() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get circulation => $_getI64(8);
  @$pb.TagNumber(9)
  set circulation($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasCirculation() => $_has(8);
  @$pb.TagNumber(9)
  void clearCirculation() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get feeSubsCount => $_getI64(9);
  @$pb.TagNumber(10)
  set feeSubsCount($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasFeeSubsCount() => $_has(9);
  @$pb.TagNumber(10)
  void clearFeeSubsCount() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get feeSubsAmount => $_getI64(10);
  @$pb.TagNumber(11)
  set feeSubsAmount($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasFeeSubsAmount() => $_has(10);
  @$pb.TagNumber(11)
  void clearFeeSubsAmount() => clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get signupRewardsCount => $_getI64(11);
  @$pb.TagNumber(12)
  set signupRewardsCount($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasSignupRewardsCount() => $_has(11);
  @$pb.TagNumber(12)
  void clearSignupRewardsCount() => clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get signupRewardsAmount => $_getI64(12);
  @$pb.TagNumber(13)
  set signupRewardsAmount($fixnum.Int64 v) { $_setInt64(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasSignupRewardsAmount() => $_has(12);
  @$pb.TagNumber(13)
  void clearSignupRewardsAmount() => clearField(13);

  @$pb.TagNumber(14)
  $fixnum.Int64 get referralRewardsCount => $_getI64(13);
  @$pb.TagNumber(14)
  set referralRewardsCount($fixnum.Int64 v) { $_setInt64(13, v); }
  @$pb.TagNumber(14)
  $core.bool hasReferralRewardsCount() => $_has(13);
  @$pb.TagNumber(14)
  void clearReferralRewardsCount() => clearField(14);

  @$pb.TagNumber(15)
  $fixnum.Int64 get referralRewardsAmount => $_getI64(14);
  @$pb.TagNumber(15)
  set referralRewardsAmount($fixnum.Int64 v) { $_setInt64(14, v); }
  @$pb.TagNumber(15)
  $core.bool hasReferralRewardsAmount() => $_has(14);
  @$pb.TagNumber(15)
  void clearReferralRewardsAmount() => clearField(15);

  @$pb.TagNumber(16)
  $fixnum.Int64 get validatorRewardsCount => $_getI64(15);
  @$pb.TagNumber(16)
  set validatorRewardsCount($fixnum.Int64 v) { $_setInt64(15, v); }
  @$pb.TagNumber(16)
  $core.bool hasValidatorRewardsCount() => $_has(15);
  @$pb.TagNumber(16)
  void clearValidatorRewardsCount() => clearField(16);

  @$pb.TagNumber(17)
  $fixnum.Int64 get validatorRewardsAmount => $_getI64(16);
  @$pb.TagNumber(17)
  set validatorRewardsAmount($fixnum.Int64 v) { $_setInt64(16, v); }
  @$pb.TagNumber(17)
  $core.bool hasValidatorRewardsAmount() => $_has(16);
  @$pb.TagNumber(17)
  void clearValidatorRewardsAmount() => clearField(17);

  @$pb.TagNumber(18)
  $fixnum.Int64 get updateUserTransactionsCount => $_getI64(17);
  @$pb.TagNumber(18)
  set updateUserTransactionsCount($fixnum.Int64 v) { $_setInt64(17, v); }
  @$pb.TagNumber(18)
  $core.bool hasUpdateUserTransactionsCount() => $_has(17);
  @$pb.TagNumber(18)
  void clearUpdateUserTransactionsCount() => clearField(18);

  @$pb.TagNumber(19)
  $core.double get exchangeRate => $_getN(18);
  @$pb.TagNumber(19)
  set exchangeRate($core.double v) { $_setDouble(18, v); }
  @$pb.TagNumber(19)
  $core.bool hasExchangeRate() => $_has(18);
  @$pb.TagNumber(19)
  void clearExchangeRate() => clearField(19);

  @$pb.TagNumber(20)
  $fixnum.Int64 get causesRewardsAmount => $_getI64(19);
  @$pb.TagNumber(20)
  set causesRewardsAmount($fixnum.Int64 v) { $_setInt64(19, v); }
  @$pb.TagNumber(20)
  $core.bool hasCausesRewardsAmount() => $_has(19);
  @$pb.TagNumber(20)
  void clearCausesRewardsAmount() => clearField(20);
}

class BlockEvent extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BlockEvent', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'karma_coin.core_types'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'height', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blockHash', $pb.PbFieldType.OY)
    ..pc<TransactionEvent>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactionsEvents', $pb.PbFieldType.PM, subBuilder: TransactionEvent.create)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'paymentsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'appreciationsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'userUpdatesCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'feesAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signupRewardsAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardsAmount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'referralRewardsCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(13, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'reward', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  BlockEvent._() : super();
  factory BlockEvent({
    $fixnum.Int64? timestamp,
    $fixnum.Int64? height,
    $core.List<$core.int>? blockHash,
    $core.Iterable<TransactionEvent>? transactionsEvents,
    $fixnum.Int64? signupsCount,
    $fixnum.Int64? paymentsCount,
    $fixnum.Int64? appreciationsCount,
    $fixnum.Int64? userUpdatesCount,
    $fixnum.Int64? feesAmount,
    $fixnum.Int64? signupRewardsAmount,
    $fixnum.Int64? referralRewardsAmount,
    $fixnum.Int64? referralRewardsCount,
    $fixnum.Int64? reward,
  }) {
    final _result = create();
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (height != null) {
      _result.height = height;
    }
    if (blockHash != null) {
      _result.blockHash = blockHash;
    }
    if (transactionsEvents != null) {
      _result.transactionsEvents.addAll(transactionsEvents);
    }
    if (signupsCount != null) {
      _result.signupsCount = signupsCount;
    }
    if (paymentsCount != null) {
      _result.paymentsCount = paymentsCount;
    }
    if (appreciationsCount != null) {
      _result.appreciationsCount = appreciationsCount;
    }
    if (userUpdatesCount != null) {
      _result.userUpdatesCount = userUpdatesCount;
    }
    if (feesAmount != null) {
      _result.feesAmount = feesAmount;
    }
    if (signupRewardsAmount != null) {
      _result.signupRewardsAmount = signupRewardsAmount;
    }
    if (referralRewardsAmount != null) {
      _result.referralRewardsAmount = referralRewardsAmount;
    }
    if (referralRewardsCount != null) {
      _result.referralRewardsCount = referralRewardsCount;
    }
    if (reward != null) {
      _result.reward = reward;
    }
    return _result;
  }
  factory BlockEvent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BlockEvent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BlockEvent clone() => BlockEvent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BlockEvent copyWith(void Function(BlockEvent) updates) => super.copyWith((message) => updates(message as BlockEvent)) as BlockEvent; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BlockEvent create() => BlockEvent._();
  BlockEvent createEmptyInstance() => create();
  static $pb.PbList<BlockEvent> createRepeated() => $pb.PbList<BlockEvent>();
  @$core.pragma('dart2js:noInline')
  static BlockEvent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BlockEvent>(create);
  static BlockEvent? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get timestamp => $_getI64(0);
  @$pb.TagNumber(1)
  set timestamp($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTimestamp() => $_has(0);
  @$pb.TagNumber(1)
  void clearTimestamp() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get height => $_getI64(1);
  @$pb.TagNumber(2)
  set height($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasHeight() => $_has(1);
  @$pb.TagNumber(2)
  void clearHeight() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get blockHash => $_getN(2);
  @$pb.TagNumber(3)
  set blockHash($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasBlockHash() => $_has(2);
  @$pb.TagNumber(3)
  void clearBlockHash() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<TransactionEvent> get transactionsEvents => $_getList(3);

  @$pb.TagNumber(5)
  $fixnum.Int64 get signupsCount => $_getI64(4);
  @$pb.TagNumber(5)
  set signupsCount($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasSignupsCount() => $_has(4);
  @$pb.TagNumber(5)
  void clearSignupsCount() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get paymentsCount => $_getI64(5);
  @$pb.TagNumber(6)
  set paymentsCount($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasPaymentsCount() => $_has(5);
  @$pb.TagNumber(6)
  void clearPaymentsCount() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get appreciationsCount => $_getI64(6);
  @$pb.TagNumber(7)
  set appreciationsCount($fixnum.Int64 v) { $_setInt64(6, v); }
  @$pb.TagNumber(7)
  $core.bool hasAppreciationsCount() => $_has(6);
  @$pb.TagNumber(7)
  void clearAppreciationsCount() => clearField(7);

  @$pb.TagNumber(8)
  $fixnum.Int64 get userUpdatesCount => $_getI64(7);
  @$pb.TagNumber(8)
  set userUpdatesCount($fixnum.Int64 v) { $_setInt64(7, v); }
  @$pb.TagNumber(8)
  $core.bool hasUserUpdatesCount() => $_has(7);
  @$pb.TagNumber(8)
  void clearUserUpdatesCount() => clearField(8);

  @$pb.TagNumber(9)
  $fixnum.Int64 get feesAmount => $_getI64(8);
  @$pb.TagNumber(9)
  set feesAmount($fixnum.Int64 v) { $_setInt64(8, v); }
  @$pb.TagNumber(9)
  $core.bool hasFeesAmount() => $_has(8);
  @$pb.TagNumber(9)
  void clearFeesAmount() => clearField(9);

  @$pb.TagNumber(10)
  $fixnum.Int64 get signupRewardsAmount => $_getI64(9);
  @$pb.TagNumber(10)
  set signupRewardsAmount($fixnum.Int64 v) { $_setInt64(9, v); }
  @$pb.TagNumber(10)
  $core.bool hasSignupRewardsAmount() => $_has(9);
  @$pb.TagNumber(10)
  void clearSignupRewardsAmount() => clearField(10);

  @$pb.TagNumber(11)
  $fixnum.Int64 get referralRewardsAmount => $_getI64(10);
  @$pb.TagNumber(11)
  set referralRewardsAmount($fixnum.Int64 v) { $_setInt64(10, v); }
  @$pb.TagNumber(11)
  $core.bool hasReferralRewardsAmount() => $_has(10);
  @$pb.TagNumber(11)
  void clearReferralRewardsAmount() => clearField(11);

  @$pb.TagNumber(12)
  $fixnum.Int64 get referralRewardsCount => $_getI64(11);
  @$pb.TagNumber(12)
  set referralRewardsCount($fixnum.Int64 v) { $_setInt64(11, v); }
  @$pb.TagNumber(12)
  $core.bool hasReferralRewardsCount() => $_has(11);
  @$pb.TagNumber(12)
  void clearReferralRewardsCount() => clearField(12);

  @$pb.TagNumber(13)
  $fixnum.Int64 get reward => $_getI64(12);
  @$pb.TagNumber(13)
  set reward($fixnum.Int64 v) { $_setInt64(12, v); }
  @$pb.TagNumber(13)
  $core.bool hasReward() => $_has(12);
  @$pb.TagNumber(13)
  void clearReward() => clearField(13);
}

