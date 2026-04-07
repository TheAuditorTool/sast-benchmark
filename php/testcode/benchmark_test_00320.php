<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00320(BenchmarkRequest $req): BenchmarkResponse {
    $id = intval($req->param('id'));
    $urls = [1 => '/home', 2 => '/profile', 3 => '/settings'];
    $url = $urls[$id] ?? '/';
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
