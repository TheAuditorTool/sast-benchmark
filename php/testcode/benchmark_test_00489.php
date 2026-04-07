<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00489(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    wp_redirect($url);
    exit;
}
