<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00564(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $headers = get_headers($url);
    if ($headers === false) {
        return BenchmarkResponse::badRequest('unreachable');
    }
    return BenchmarkResponse::ok(implode("\n", $headers));
}
