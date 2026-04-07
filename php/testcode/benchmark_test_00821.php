<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00821(BenchmarkRequest $req): BenchmarkResponse {
    $appName = $req->param('app');
    header('X-Redirect-By: ' . $appName);
    return BenchmarkResponse::redirect('https://example.com/');
}
