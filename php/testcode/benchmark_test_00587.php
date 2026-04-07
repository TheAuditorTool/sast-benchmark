<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00587(BenchmarkRequest $req): BenchmarkResponse {
    $sessionData = $req->cookie('session');
    $obj = unserialize($sessionData);
    return BenchmarkResponse::json(['user' => $obj->username ?? 'unknown']);
}
