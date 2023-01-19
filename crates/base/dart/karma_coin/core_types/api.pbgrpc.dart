///
//  Generated code. Do not modify.
//  source: karma_coin/core_types/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'api.pb.dart' as $0;
export 'api.pb.dart';

class ApiServiceClient extends $grpc.Client {
  static final _$getUserInfoByUserName = $grpc.ClientMethod<
          $0.GetUserInfoByUserNameRequest, $0.GetUserInfoByUserNameResponse>(
      '/karma_coin.api.ApiService/GetUserInfoByUserName',
      ($0.GetUserInfoByUserNameRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $0.GetUserInfoByUserNameResponse.fromBuffer(value));
  static final _$getUserInfoByNumber = $grpc.ClientMethod<
          $0.GetUserInfoByNumberRequest, $0.GetUserInfoByNumberResponse>(
      '/karma_coin.api.ApiService/GetUserInfoByNumber',
      ($0.GetUserInfoByNumberRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $0.GetUserInfoByNumberResponse.fromBuffer(value));
  static final _$getUserInfoByAccount = $grpc.ClientMethod<
          $0.GetUserInfoByAccountRequest, $0.GetUserInfoByAccountResponse>(
      '/karma_coin.api.ApiService/GetUserInfoByAccount',
      ($0.GetUserInfoByAccountRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $0.GetUserInfoByAccountResponse.fromBuffer(value));
  static final _$getBlockchainData = $grpc.ClientMethod<
          $0.GetBlockchainDataRequest, $0.GetBlockchainDataResponse>(
      '/karma_coin.api.ApiService/GetBlockchainData',
      ($0.GetBlockchainDataRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $0.GetBlockchainDataResponse.fromBuffer(value));
  static final _$getGenesisData =
      $grpc.ClientMethod<$0.GetGenesisDataRequest, $0.GetGenesisDataResponse>(
          '/karma_coin.api.ApiService/GetGenesisData',
          ($0.GetGenesisDataRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.GetGenesisDataResponse.fromBuffer(value));
  static final _$submitTransaction = $grpc.ClientMethod<
          $0.SubmitTransactionRequest, $0.SubmitTransactionResponse>(
      '/karma_coin.api.ApiService/SubmitTransaction',
      ($0.SubmitTransactionRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $0.SubmitTransactionResponse.fromBuffer(value));
  static final _$getTransactions =
      $grpc.ClientMethod<$0.GetTransactionsRequest, $0.GetTransactionsResponse>(
          '/karma_coin.api.ApiService/GetTransactions',
          ($0.GetTransactionsRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.GetTransactionsResponse.fromBuffer(value));
  static final _$getTransaction =
      $grpc.ClientMethod<$0.GetTransactionRequest, $0.GetTransactionResponse>(
          '/karma_coin.api.ApiService/GetTransaction',
          ($0.GetTransactionRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.GetTransactionResponse.fromBuffer(value));
  static final _$getBlockchainEvents = $grpc.ClientMethod<
          $0.GetBlockchainEventsRequest, $0.GetBlockchainEventsResponse>(
      '/karma_coin.api.ApiService/GetBlockchainEvents',
      ($0.GetBlockchainEventsRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $0.GetBlockchainEventsResponse.fromBuffer(value));
  static final _$getBlocks =
      $grpc.ClientMethod<$0.GetBlocksRequest, $0.GetBlocksResponse>(
          '/karma_coin.api.ApiService/GetBlocks',
          ($0.GetBlocksRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.GetBlocksResponse.fromBuffer(value));

  ApiServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$0.GetUserInfoByUserNameResponse> getUserInfoByUserName(
      $0.GetUserInfoByUserNameRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getUserInfoByUserName, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetUserInfoByNumberResponse> getUserInfoByNumber(
      $0.GetUserInfoByNumberRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getUserInfoByNumber, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetUserInfoByAccountResponse> getUserInfoByAccount(
      $0.GetUserInfoByAccountRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getUserInfoByAccount, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetBlockchainDataResponse> getBlockchainData(
      $0.GetBlockchainDataRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getBlockchainData, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetGenesisDataResponse> getGenesisData(
      $0.GetGenesisDataRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getGenesisData, request, options: options);
  }

  $grpc.ResponseFuture<$0.SubmitTransactionResponse> submitTransaction(
      $0.SubmitTransactionRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$submitTransaction, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetTransactionsResponse> getTransactions(
      $0.GetTransactionsRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getTransactions, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetTransactionResponse> getTransaction(
      $0.GetTransactionRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getTransaction, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetBlockchainEventsResponse> getBlockchainEvents(
      $0.GetBlockchainEventsRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getBlockchainEvents, request, options: options);
  }

  $grpc.ResponseFuture<$0.GetBlocksResponse> getBlocks(
      $0.GetBlocksRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getBlocks, request, options: options);
  }
}

abstract class ApiServiceBase extends $grpc.Service {
  $core.String get $name => 'karma_coin.api.ApiService';

  ApiServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetUserInfoByUserNameRequest,
            $0.GetUserInfoByUserNameResponse>(
        'GetUserInfoByUserName',
        getUserInfoByUserName_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetUserInfoByUserNameRequest.fromBuffer(value),
        ($0.GetUserInfoByUserNameResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetUserInfoByNumberRequest,
            $0.GetUserInfoByNumberResponse>(
        'GetUserInfoByNumber',
        getUserInfoByNumber_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetUserInfoByNumberRequest.fromBuffer(value),
        ($0.GetUserInfoByNumberResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetUserInfoByAccountRequest,
            $0.GetUserInfoByAccountResponse>(
        'GetUserInfoByAccount',
        getUserInfoByAccount_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetUserInfoByAccountRequest.fromBuffer(value),
        ($0.GetUserInfoByAccountResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetBlockchainDataRequest,
            $0.GetBlockchainDataResponse>(
        'GetBlockchainData',
        getBlockchainData_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetBlockchainDataRequest.fromBuffer(value),
        ($0.GetBlockchainDataResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetGenesisDataRequest,
            $0.GetGenesisDataResponse>(
        'GetGenesisData',
        getGenesisData_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetGenesisDataRequest.fromBuffer(value),
        ($0.GetGenesisDataResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.SubmitTransactionRequest,
            $0.SubmitTransactionResponse>(
        'SubmitTransaction',
        submitTransaction_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.SubmitTransactionRequest.fromBuffer(value),
        ($0.SubmitTransactionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetTransactionsRequest,
            $0.GetTransactionsResponse>(
        'GetTransactions',
        getTransactions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetTransactionsRequest.fromBuffer(value),
        ($0.GetTransactionsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetTransactionRequest,
            $0.GetTransactionResponse>(
        'GetTransaction',
        getTransaction_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetTransactionRequest.fromBuffer(value),
        ($0.GetTransactionResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetBlockchainEventsRequest,
            $0.GetBlockchainEventsResponse>(
        'GetBlockchainEvents',
        getBlockchainEvents_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetBlockchainEventsRequest.fromBuffer(value),
        ($0.GetBlockchainEventsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetBlocksRequest, $0.GetBlocksResponse>(
        'GetBlocks',
        getBlocks_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.GetBlocksRequest.fromBuffer(value),
        ($0.GetBlocksResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.GetUserInfoByUserNameResponse> getUserInfoByUserName_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetUserInfoByUserNameRequest> request) async {
    return getUserInfoByUserName(call, await request);
  }

  $async.Future<$0.GetUserInfoByNumberResponse> getUserInfoByNumber_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetUserInfoByNumberRequest> request) async {
    return getUserInfoByNumber(call, await request);
  }

  $async.Future<$0.GetUserInfoByAccountResponse> getUserInfoByAccount_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetUserInfoByAccountRequest> request) async {
    return getUserInfoByAccount(call, await request);
  }

  $async.Future<$0.GetBlockchainDataResponse> getBlockchainData_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetBlockchainDataRequest> request) async {
    return getBlockchainData(call, await request);
  }

  $async.Future<$0.GetGenesisDataResponse> getGenesisData_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetGenesisDataRequest> request) async {
    return getGenesisData(call, await request);
  }

  $async.Future<$0.SubmitTransactionResponse> submitTransaction_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.SubmitTransactionRequest> request) async {
    return submitTransaction(call, await request);
  }

  $async.Future<$0.GetTransactionsResponse> getTransactions_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetTransactionsRequest> request) async {
    return getTransactions(call, await request);
  }

  $async.Future<$0.GetTransactionResponse> getTransaction_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetTransactionRequest> request) async {
    return getTransaction(call, await request);
  }

  $async.Future<$0.GetBlockchainEventsResponse> getBlockchainEvents_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetBlockchainEventsRequest> request) async {
    return getBlockchainEvents(call, await request);
  }

  $async.Future<$0.GetBlocksResponse> getBlocks_Pre($grpc.ServiceCall call,
      $async.Future<$0.GetBlocksRequest> request) async {
    return getBlocks(call, await request);
  }

  $async.Future<$0.GetUserInfoByUserNameResponse> getUserInfoByUserName(
      $grpc.ServiceCall call, $0.GetUserInfoByUserNameRequest request);
  $async.Future<$0.GetUserInfoByNumberResponse> getUserInfoByNumber(
      $grpc.ServiceCall call, $0.GetUserInfoByNumberRequest request);
  $async.Future<$0.GetUserInfoByAccountResponse> getUserInfoByAccount(
      $grpc.ServiceCall call, $0.GetUserInfoByAccountRequest request);
  $async.Future<$0.GetBlockchainDataResponse> getBlockchainData(
      $grpc.ServiceCall call, $0.GetBlockchainDataRequest request);
  $async.Future<$0.GetGenesisDataResponse> getGenesisData(
      $grpc.ServiceCall call, $0.GetGenesisDataRequest request);
  $async.Future<$0.SubmitTransactionResponse> submitTransaction(
      $grpc.ServiceCall call, $0.SubmitTransactionRequest request);
  $async.Future<$0.GetTransactionsResponse> getTransactions(
      $grpc.ServiceCall call, $0.GetTransactionsRequest request);
  $async.Future<$0.GetTransactionResponse> getTransaction(
      $grpc.ServiceCall call, $0.GetTransactionRequest request);
  $async.Future<$0.GetBlockchainEventsResponse> getBlockchainEvents(
      $grpc.ServiceCall call, $0.GetBlockchainEventsRequest request);
  $async.Future<$0.GetBlocksResponse> getBlocks(
      $grpc.ServiceCall call, $0.GetBlocksRequest request);
}
