<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00186(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $info = stat($userPath);
    return BenchmarkResponse::ok(json_encode(['size' => $info['size'] ?? 0]));
}
