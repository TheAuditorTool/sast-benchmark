<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00128(BenchmarkRequest $req): BenchmarkResponse {
    $apiKey = $req->cookie('api_key');
    if (empty($apiKey) || !validateApiKey($apiKey)) {
        return BenchmarkResponse::error('unauthorized');
    }
    $result = performApiAction($req->param('action'));
    return BenchmarkResponse::json(['result' => $result]);
}
