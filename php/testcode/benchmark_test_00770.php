<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00770(BenchmarkRequest $req): BenchmarkResponse {
    $pubKey = openssl_pkey_get_public(file_get_contents('/etc/app/encrypt.pub'));
    openssl_seal($req->param('data'), $sealed, $envKeys, [$pubKey]);
    return BenchmarkResponse::ok(base64_encode($sealed));
}
