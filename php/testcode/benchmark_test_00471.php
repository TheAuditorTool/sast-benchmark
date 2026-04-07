<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00471(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $html = '<meta http-equiv="refresh" content="0;url=' . $url . '">';
    return BenchmarkResponse::html($html);
}
