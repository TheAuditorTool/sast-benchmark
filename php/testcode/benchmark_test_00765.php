<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00765(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $check = crc32($token);
    if ($check !== (int)$req->param('checksum')) {
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('valid');
}
