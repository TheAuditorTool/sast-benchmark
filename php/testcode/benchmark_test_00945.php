<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00945(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    $fh = fopen("/uploads/" . $filename, "r");
    $content = fread($fh, filesize("/uploads/" . $filename));
    fclose($fh);
    return BenchmarkResponse::ok($content);
}
