<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00459(BenchmarkRequest $req): BenchmarkResponse {
    $jobId = (int)$req->param('job_id');
    $db = new PDO('sqlite:/tmp/jobs.db');
    $stmt = $db->prepare('SELECT fetch_url FROM scheduled_jobs WHERE id = ?');
    $stmt->execute([$jobId]);
    $fetchUrl = $stmt->fetchColumn();
    $content = file_get_contents($fetchUrl);
    return BenchmarkResponse::ok($content);
}
