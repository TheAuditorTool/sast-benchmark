<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00267(BenchmarkRequest $req): BenchmarkResponse {
    $chars = 'abcdefghijklmnopqrstuvwxyz0123456789';
    $token = substr(str_shuffle($chars), 0, 16);
    return BenchmarkResponse::json(['api_key' => $token]);
}
