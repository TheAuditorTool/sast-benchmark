<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00589(BenchmarkRequest $req): BenchmarkResponse {
    $userId = $req->cookie('user_id');
    $token = md5($userId);
    if ($req->post('csrf_token') !== $token) {
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    return BenchmarkResponse::ok('Action performed');
}
