<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00108(BenchmarkRequest $req): BenchmarkResponse {
    $redirect = $req->param('url');
    header('Refresh: 5; url=' . $redirect);
    return BenchmarkResponse::ok('redirecting shortly');
}
