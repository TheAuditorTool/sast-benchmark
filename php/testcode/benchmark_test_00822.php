<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00822(BenchmarkRequest $req): BenchmarkResponse {
    define('AWS_ACCESS_KEY', 'AKIAIOSFODNN7EXAMPLE');
    define('AWS_SECRET', 'wJalrXUtnFEMI/K7MDENG');
    $bucket = $req->param('bucket');
    $headers = [
        'X-Amz-Access-Key: ' . AWS_ACCESS_KEY,
        'X-Amz-Secret: ' . AWS_SECRET,
    ];
    return BenchmarkResponse::ok('bucket: ' . $bucket);
}
