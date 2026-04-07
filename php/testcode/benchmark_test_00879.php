<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00879(BenchmarkRequest $req): BenchmarkResponse {
    $varname = $req->param('field');
    $$varname = $req->param('value');
    $data = compact($varname);
    return BenchmarkResponse::json($data);
}
