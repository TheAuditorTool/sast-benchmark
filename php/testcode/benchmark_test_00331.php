<?php
require_once __DIR__ . '/shared.php';

define('ASSETS_DIR', '/var/app/assets/');

function benchmarkTest00331(BenchmarkRequest $req): BenchmarkResponse {
    $filename = basename($req->param('file'));
    $content = file_get_contents(ASSETS_DIR . $filename);
    return BenchmarkResponse::ok($content);
}
