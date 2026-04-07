<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00503(BenchmarkRequest $req): BenchmarkResponse {
    $realm = $req->param('realm');
    header('WWW-Authenticate: Basic realm="' . $realm . '"');
    return BenchmarkResponse::error('Unauthorized', 401);
}
