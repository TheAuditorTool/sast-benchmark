<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01119(BenchmarkRequest $req): BenchmarkResponse {
    $userId = $req->param('id');
    header('X-User-Id: ' . intval($userId));
    return BenchmarkResponse::ok('user id set');
}
