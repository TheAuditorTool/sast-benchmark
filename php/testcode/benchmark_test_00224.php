<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00224(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    echo "<meta http-equiv='refresh' content='0;url=$url'>";
    return BenchmarkResponse::ok('');
}
