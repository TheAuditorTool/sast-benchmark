<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00035(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    wp_safe_redirect($url, 302, 'my-plugin');
    exit;
}
