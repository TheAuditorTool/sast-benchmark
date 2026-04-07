<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00659(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (preg_match('/^https?:\/\/example\.com/', $url)) {
        header('Location: ' . $url);
    }
    return BenchmarkResponse::ok('');
}
