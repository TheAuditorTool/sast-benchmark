<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00024(BenchmarkRequest $req): BenchmarkResponse {
    $userPattern = $req->param('pattern');
    $files = glob($userPattern);
    return BenchmarkResponse::ok(implode("\n", $files ?: []));
}
