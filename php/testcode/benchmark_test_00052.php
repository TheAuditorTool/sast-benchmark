<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00052(BenchmarkRequest $req): BenchmarkResponse {
    $metaUrl = 'http://169.254.169.254/latest/meta-data/iam/security-credentials/';
    $content = file_get_contents($metaUrl . $req->param('path'));
    return BenchmarkResponse::ok($content);
}
