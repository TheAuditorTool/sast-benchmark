<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00358(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $allowed = ['readme.txt', 'license.txt', 'changelog.txt'];
    if (!in_array($filename, $allowed, true)) {
        return BenchmarkResponse::badRequest('not allowed');
    }
    $content = file_get_contents('/var/app/public/' . $filename);
    return BenchmarkResponse::ok($content);
}
