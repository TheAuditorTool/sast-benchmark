<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00064(BenchmarkRequest $req): BenchmarkResponse {
    $encryptedToken = $req->cookie('csrf_enc');
    $submitted = $req->post('csrf_token');
    $decrypted = decryptToken($encryptedToken);
    if (!hash_equals($decrypted, (string) $submitted)) {
        return BenchmarkResponse::badRequest('CSRF token mismatch');
    }
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
