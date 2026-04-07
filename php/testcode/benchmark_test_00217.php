<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00217(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    if (!preg_match('/^[a-zA-Z0-9_-]+\.(jpg|png|pdf)$/', $filename)) {
        return BenchmarkResponse::badRequest('invalid filename');
    }
    $content = file_get_contents('/var/app/files/' . $filename);
    return BenchmarkResponse::ok($content);
}
