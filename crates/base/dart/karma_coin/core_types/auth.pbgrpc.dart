///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/auth.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'auth.pb.dart' as $1;
export 'auth.pb.dart';

class AuthServiceClient extends $grpc.Client {
  static final _$authenticate =
      $grpc.ClientMethod<$1.AuthRequest, $1.AuthResponse>(
          '/karma_coin.auth.AuthService/Authenticate',
          ($1.AuthRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $1.AuthResponse.fromBuffer(value));

  AuthServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$1.AuthResponse> authenticate($1.AuthRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$authenticate, request, options: options);
  }
}

abstract class AuthServiceBase extends $grpc.Service {
  $core.String get $name => 'karma_coin.auth.AuthService';

  AuthServiceBase() {
    $addMethod($grpc.ServiceMethod<$1.AuthRequest, $1.AuthResponse>(
        'Authenticate',
        authenticate_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.AuthRequest.fromBuffer(value),
        ($1.AuthResponse value) => value.writeToBuffer()));
  }

  $async.Future<$1.AuthResponse> authenticate_Pre(
      $grpc.ServiceCall call, $async.Future<$1.AuthRequest> request) async {
    return authenticate(call, await request);
  }

  $async.Future<$1.AuthResponse> authenticate(
      $grpc.ServiceCall call, $1.AuthRequest request);
}
