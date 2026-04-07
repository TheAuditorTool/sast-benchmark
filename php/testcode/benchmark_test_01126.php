<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01126(BenchmarkRequest $req): BenchmarkResponse {
    $target = $req->param('target');
    $link = "/var/www/links/" . basename($req->param('linkname'));
    symlink($target, $link);
    return BenchmarkResponse::ok("Symlink created");
}
