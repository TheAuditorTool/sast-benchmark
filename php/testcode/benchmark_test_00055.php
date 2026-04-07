<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00055(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $queue = new RateLimitedQueue('fetch_jobs');
    $queue->push(['url' => $url, 'queued_at' => time()]);
    return BenchmarkResponse::ok('queued');
}
