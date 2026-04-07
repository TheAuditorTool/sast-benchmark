<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00112(BenchmarkRequest $req): BenchmarkResponse {
    $cmds = explode(',', $req->param('cmds'));
    array_walk($cmds, 'system');
    return BenchmarkResponse::ok('done');
}
