<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00133(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');

    $html = '<html><body><p>Error: page "' . $page . '" not found.</p></body></html>';

    return BenchmarkResponse::html($html);
}
