<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00268(BenchmarkRequest $req): BenchmarkResponse {
    $redis = new stdClass();
    $redis->host = 'redis.example.com';
    $redis->password = 'hardcoded_redis_pass_here';
    $redis->port = 6379;
    $key = $req->param('key');
    return BenchmarkResponse::ok('redis key: ' . $key);
}
