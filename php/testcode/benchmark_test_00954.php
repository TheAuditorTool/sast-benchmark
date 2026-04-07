<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00954(BenchmarkRequest $req): BenchmarkResponse {
    $raw = file_get_contents('/var/cache/app.dat');
    $config = unserialize($raw);
    return BenchmarkResponse::json(['cache_ttl' => $config['ttl'] ?? 3600]);
}
