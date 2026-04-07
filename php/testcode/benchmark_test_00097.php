<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00097(BenchmarkRequest $req): BenchmarkResponse {
    define('API_KEY', 'REPLACE_IN_DEPLOY');
    $endpoint = $req->param('endpoint');
    $ch = curl_init($endpoint);
    curl_setopt($ch, CURLOPT_HTTPHEADER, ['X-API-Key: ' . API_KEY]);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    return BenchmarkResponse::ok((string)$result);
}
