<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00960(BenchmarkRequest $req): BenchmarkResponse {
    $userDir = $req->param('dir');
    $files = scandir($userDir);
    return BenchmarkResponse::ok(implode("\n", $files ?: []));
}
