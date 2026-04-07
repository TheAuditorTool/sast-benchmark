<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_redis_auth
function hardcodedcreds025(BenchmarkRequest $req): BenchmarkResponse {
    $redis = new stdClass();
    $redis->host = 'redis.example.com';
    $redis->password = 'hardcoded_redis_pass_here'; // vuln-code-snippet vuln-line php_hardcoded_redis_auth
    $redis->port = 6379;
    $key = $req->param('key');
    return BenchmarkResponse::ok('redis key: ' . $key);
}
// vuln-code-snippet end php_hardcoded_redis_auth
