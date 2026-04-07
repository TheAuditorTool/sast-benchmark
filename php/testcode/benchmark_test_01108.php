<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01108(BenchmarkRequest $req): BenchmarkResponse {
    $queuePath = '/var/run/internal_queue/next_job.bin';
    $raw = file_get_contents($queuePath);
    $job = unserialize($raw);
    processJob($job);
    return BenchmarkResponse::ok('job processed');
}
