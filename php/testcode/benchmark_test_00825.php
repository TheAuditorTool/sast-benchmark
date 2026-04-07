<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00825(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    $blockedSchemes = ['data', 'php', 'expect', 'http', 'https', 'ftp'];
    $scheme = parse_url($path, PHP_URL_SCHEME);
    if ($scheme !== null && in_array(strtolower($scheme), $blockedSchemes, true)) {
        return BenchmarkResponse::badRequest("blocked URL scheme: " . $scheme);
    }
    $safe = basename($path);
    include("templates/" . $safe);
    return BenchmarkResponse::ok("content loaded");
}
