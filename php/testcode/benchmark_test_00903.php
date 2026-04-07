<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00903(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    include("uploads/" . $file . ".php");
    return BenchmarkResponse::ok("file loaded");
}
