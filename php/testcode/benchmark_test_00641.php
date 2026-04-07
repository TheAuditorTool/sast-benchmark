<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00641(BenchmarkRequest $req): BenchmarkResponse {
    $plugin = $req->param('plugin');
    $sig = $req->header('X-Plugin-Sig');
    if (!openssl_verify($plugin, base64_decode($sig), file_get_contents('/etc/app/plugin.pub'), OPENSSL_ALGO_SHA256)) {
        return BenchmarkResponse::badRequest('invalid sig');
    }
    return BenchmarkResponse::ok('verified');
}
