<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00123(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $allowed = ['/', '/dashboard', '/profile'];
    if (in_array($url, $allowed, true)) {
        header("Location: " . $url);
        return BenchmarkResponse::ok('Redirecting...');
    }
    return BenchmarkResponse::badRequest('Invalid redirect target');
}
