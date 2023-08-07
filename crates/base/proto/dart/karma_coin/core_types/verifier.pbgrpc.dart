///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/verifier.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'verifier.pb.dart' as $2;
export 'verifier.pb.dart';

class VerifierServiceClient extends $grpc.Client {
  static final _$verifyNumber =
      $grpc.ClientMethod<$2.VerifyNumberRequest, $2.VerifyNumberResponse>(
          '/karma_coin.verifier.VerifierService/VerifyNumber',
          ($2.VerifyNumberRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $2.VerifyNumberResponse.fromBuffer(value));
  static final _$verifyNumberEx =
      $grpc.ClientMethod<$2.VerifyNumberRequestEx, $2.VerifyNumberResponse>(
          '/karma_coin.verifier.VerifierService/VerifyNumberEx',
          ($2.VerifyNumberRequestEx value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $2.VerifyNumberResponse.fromBuffer(value));
  static final _$sendVerificationCode = $grpc.ClientMethod<
          $2.SendVerificationCodeRequest, $2.SendVerificationCodeResponse>(
      '/karma_coin.verifier.VerifierService/SendVerificationCode',
      ($2.SendVerificationCodeRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $2.SendVerificationCodeResponse.fromBuffer(value));

  VerifierServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$2.VerifyNumberResponse> verifyNumber(
      $2.VerifyNumberRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$verifyNumber, request, options: options);
  }

  $grpc.ResponseFuture<$2.VerifyNumberResponse> verifyNumberEx(
      $2.VerifyNumberRequestEx request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$verifyNumberEx, request, options: options);
  }

  $grpc.ResponseFuture<$2.SendVerificationCodeResponse> sendVerificationCode(
      $2.SendVerificationCodeRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$sendVerificationCode, request, options: options);
  }
}

abstract class VerifierServiceBase extends $grpc.Service {
  $core.String get $name => 'karma_coin.verifier.VerifierService';

  VerifierServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$2.VerifyNumberRequest, $2.VerifyNumberResponse>(
            'VerifyNumber',
            verifyNumber_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $2.VerifyNumberRequest.fromBuffer(value),
            ($2.VerifyNumberResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$2.VerifyNumberRequestEx, $2.VerifyNumberResponse>(
            'VerifyNumberEx',
            verifyNumberEx_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $2.VerifyNumberRequestEx.fromBuffer(value),
            ($2.VerifyNumberResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$2.SendVerificationCodeRequest,
            $2.SendVerificationCodeResponse>(
        'SendVerificationCode',
        sendVerificationCode_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $2.SendVerificationCodeRequest.fromBuffer(value),
        ($2.SendVerificationCodeResponse value) => value.writeToBuffer()));
  }

  $async.Future<$2.VerifyNumberResponse> verifyNumber_Pre(
      $grpc.ServiceCall call,
      $async.Future<$2.VerifyNumberRequest> request) async {
    return verifyNumber(call, await request);
  }

  $async.Future<$2.VerifyNumberResponse> verifyNumberEx_Pre(
      $grpc.ServiceCall call,
      $async.Future<$2.VerifyNumberRequestEx> request) async {
    return verifyNumberEx(call, await request);
  }

  $async.Future<$2.SendVerificationCodeResponse> sendVerificationCode_Pre(
      $grpc.ServiceCall call,
      $async.Future<$2.SendVerificationCodeRequest> request) async {
    return sendVerificationCode(call, await request);
  }

  $async.Future<$2.VerifyNumberResponse> verifyNumber(
      $grpc.ServiceCall call, $2.VerifyNumberRequest request);
  $async.Future<$2.VerifyNumberResponse> verifyNumberEx(
      $grpc.ServiceCall call, $2.VerifyNumberRequestEx request);
  $async.Future<$2.SendVerificationCodeResponse> sendVerificationCode(
      $grpc.ServiceCall call, $2.SendVerificationCodeRequest request);
}
