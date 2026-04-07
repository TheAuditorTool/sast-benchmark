<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00411(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    echo "<script>location.href='$url'</script>";
    return BenchmarkResponse::ok('');
}
