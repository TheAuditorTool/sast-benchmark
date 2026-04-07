<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00669(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('file');
    $filename = basename($input);
    $content = file_get_contents('/var/app/files/' . $filename);
    return BenchmarkResponse::ok($content);
}
