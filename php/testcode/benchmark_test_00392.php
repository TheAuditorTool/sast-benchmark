<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00392(BenchmarkRequest $req): BenchmarkResponse {
    $realm = $req->param('realm');
    if (!ctype_alnum($realm)) {
        return BenchmarkResponse::badRequest('invalid realm');
    }
    header('WWW-Authenticate: Basic realm="' . $realm . '"');
    return BenchmarkResponse::error('Unauthorized', 401);
}
