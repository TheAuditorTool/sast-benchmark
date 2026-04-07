<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00541(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    include($url);
    return BenchmarkResponse::ok("remote content loaded");
}
