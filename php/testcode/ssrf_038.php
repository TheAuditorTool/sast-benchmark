<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_rate_limited_queue
function ssrf038(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $queue = new RateLimitedQueue('fetch_jobs');
    $queue->push(['url' => $url, 'queued_at' => time()]); // vuln-code-snippet safe-line php_ssrf_rate_limited_queue
    return BenchmarkResponse::ok('queued');
}
// vuln-code-snippet end php_ssrf_rate_limited_queue
