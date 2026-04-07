<?php
require_once __DIR__ . '/shared.php';

function wp_redirect_unsafe(string $location): void {
    header("Location: " . $location);
}

function benchmarkTest00747(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('redirect_to');
    wp_redirect_unsafe($url);
    return BenchmarkResponse::ok('');
}
