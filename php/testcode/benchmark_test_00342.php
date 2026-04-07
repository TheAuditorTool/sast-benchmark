<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00342(BenchmarkRequest $req): BenchmarkResponse {
    $target = $req->param('goto');
    return BenchmarkResponse::redirect($target);
}
