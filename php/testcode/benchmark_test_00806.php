<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00806(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    system(escapeshellcmd("ping -c 3 " . $host));
    return BenchmarkResponse::ok("ping complete");
}
