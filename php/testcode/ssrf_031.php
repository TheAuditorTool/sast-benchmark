<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_db_cron_fetch
function ssrf031(BenchmarkRequest $req): BenchmarkResponse {
    $jobId = (int)$req->param('job_id');
    $db = new PDO('sqlite:/tmp/jobs.db');
    $stmt = $db->prepare('SELECT fetch_url FROM scheduled_jobs WHERE id = ?');
    $stmt->execute([$jobId]);
    $fetchUrl = $stmt->fetchColumn();
    $content = file_get_contents($fetchUrl); // vuln-code-snippet vuln-line php_ssrf_db_cron_fetch
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_db_cron_fetch
