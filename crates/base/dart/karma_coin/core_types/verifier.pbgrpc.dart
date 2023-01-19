///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/verifier.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'verifier.pb.dart' as $1;
import 'types.pb.dart' as $2;
export 'verifier.pb.dart';

class VerifierServiceClient extends $grpc.Client {
  static final _$verifyNumber =
      $grpc.ClientMethod<$1.VerifyNumberRequest, $2.VerifyNumberResponse>(
          '/karma_coin.verifier.VerifierService/VerifyNumber',
          ($1.VerifyNumberRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $2.VerifyNumberResponse.fromBuffer(value));

  VerifierServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$2.VerifyNumberResponse> verifyNumber(
      $1.VerifyNumberRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$verifyNumber, request, options: options);
  }
}

abstract class VerifierServiceBase extends $grpc.Service {
  $core.String get $name => 'karma_coin.verifier.VerifierService';

  VerifierServiceBase() {
    $addMethod(
        $grpc.ServiceMethod<$1.VerifyNumberRequest, $2.VerifyNumberResponse>(
            'VerifyNumber',
            verifyNumber_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $1.VerifyNumberRequest.fromBuffer(value),
            ($2.VerifyNumberResponse value) => value.writeToBuffer()));
  }

  $async.Future<$2.VerifyNumberResponse> verifyNumber_Pre(
      $grpc.ServiceCall call,
      $async.Future<$1.VerifyNumberRequest> request) async {
    return verifyNumber(call, await request);
  }

  $async.Future<$2.VerifyNumberResponse> verifyNumber(
      $grpc.ServiceCall call, $1.VerifyNumberRequest request);
}
