<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00085(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $html = '<script>window.location="' . $url . '";</script>';
    return BenchmarkResponse::html($html);
}
