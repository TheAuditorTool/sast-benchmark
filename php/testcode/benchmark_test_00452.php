<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00452(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    chmod($userPath, 0777);
    return BenchmarkResponse::ok('permissions changed');
}
