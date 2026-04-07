<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00075(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $output = shell_exec("ffmpeg -i $file -o output.mp4");
    return BenchmarkResponse::ok($output ?? '');
}
