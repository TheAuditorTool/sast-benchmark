<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00162(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    include($page . ".php");
    return BenchmarkResponse::ok("page loaded");
}
