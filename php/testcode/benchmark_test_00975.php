<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00975(BenchmarkRequest $req): BenchmarkResponse {
    $content = file_get_contents("phar:///var/app/archive.phar");
    return BenchmarkResponse::ok($content);
}
