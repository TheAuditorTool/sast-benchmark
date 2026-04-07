<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00482(BenchmarkRequest $req): BenchmarkResponse {
    $src = '/var/app/templates/default.html';
    $userDest = $req->param('dest');
    copy($src, $userDest);
    return BenchmarkResponse::ok('copied');
}
