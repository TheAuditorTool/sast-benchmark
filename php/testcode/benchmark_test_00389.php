<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00389(BenchmarkRequest $req): BenchmarkResponse {
    $stripe = new stdClass();
    $stripe->apiKey = 'sk_live_realKeyHere123456abcdef';
    $amount = (int)$req->param('amount');
    $charge = ['amount' => $amount, 'currency' => 'usd'];
    return BenchmarkResponse::json($charge);
}
