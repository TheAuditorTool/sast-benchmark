<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00160(BenchmarkRequest $req): BenchmarkResponse {
    $varname = $req->param('key');
    $value = $req->param('value');
    $$varname = $value;
    return BenchmarkResponse::ok("set $varname");
}
