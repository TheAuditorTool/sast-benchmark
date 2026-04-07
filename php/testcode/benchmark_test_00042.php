<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00042(BenchmarkRequest $req): BenchmarkResponse {
    $submitted = $req->post('token');
    $stored = $_SESSION['auth_token'] ?? '';
    if (!hash_equals($stored, $submitted)) {
        return BenchmarkResponse::badRequest('Invalid token');
    }
    return BenchmarkResponse::ok('Authenticated');
}
