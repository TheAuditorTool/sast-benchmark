<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00422(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    if (!isset($_SESSION['csrf_token'])) {
        $_SESSION['csrf_token'] = bin2hex(random_bytes(16));
    }
    $submitted = $req->post('csrf_token');
    if ($submitted === $_SESSION['csrf_token']) {
        performSensitiveAction();
        return BenchmarkResponse::ok('action done');
    }
    return BenchmarkResponse::badRequest('invalid token');
}
