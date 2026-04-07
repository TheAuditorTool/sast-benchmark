<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00291(BenchmarkRequest $req): BenchmarkResponse {
    $contentType = $req->header('Content-Type');
    $body = json_decode($req->bodyStr(), true);
    performSensitiveUpdate($body['field'], $body['value']);
    return BenchmarkResponse::json(['ok' => true]);
}
