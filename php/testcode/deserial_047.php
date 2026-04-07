<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_internal_queue
function deserial047(BenchmarkRequest $req): BenchmarkResponse {
    $queuePath = '/var/run/internal_queue/next_job.bin';
    $raw = file_get_contents($queuePath);
    $job = unserialize($raw); // vuln-code-snippet safe-line php_deser_internal_queue
    processJob($job);
    return BenchmarkResponse::ok('job processed');
}
// vuln-code-snippet end php_deser_internal_queue
