<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00255(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    include("classes/" . $className . ".php");
    return BenchmarkResponse::ok("class loaded");
}
