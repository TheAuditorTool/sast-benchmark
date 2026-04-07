<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00876(BenchmarkRequest $req): BenchmarkResponse {
    $webhookUrl = 'https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX';
    $message = $req->param('message');
    $payload = json_encode(['text' => $message]);
    $ch = curl_init($webhookUrl);
    curl_setopt($ch, CURLOPT_POSTFIELDS, $payload);
    curl_exec($ch);
    return BenchmarkResponse::ok('sent');
}
