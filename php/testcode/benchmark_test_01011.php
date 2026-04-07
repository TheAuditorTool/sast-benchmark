<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01011(BenchmarkRequest $req): BenchmarkResponse {
    $role = $req->param('role');
    $result = match ($role) {
        'admin' => 'admin access granted',
        'editor' => 'editor access',
        default => 'guest access',
    };
    return BenchmarkResponse::ok($result);
}
