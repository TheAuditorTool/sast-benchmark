<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00533(BenchmarkRequest $req): BenchmarkResponse {
    $submitted = $req->post('csrf_token');
    $stored = $_SESSION['csrf_token'] ?? '';
    if (empty($submitted) || !hash_equals($stored, $submitted)) {
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
    return BenchmarkResponse::ok('Action performed, token rotated');
}
