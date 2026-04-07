<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00344(BenchmarkRequest $req): BenchmarkResponse {
    $src = '/var/app/uploads/' . basename($req->param('src'));
    $userDest = $req->param('dest');
    rename($src, $userDest);
    return BenchmarkResponse::ok('renamed');
}
