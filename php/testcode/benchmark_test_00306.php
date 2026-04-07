<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00306(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $key = $req->param('key');
    $prefix = ini_get('session.upload_progress.prefix');
    $progressKey = $prefix . $key;
    $raw = $_SESSION[$progressKey] ?? '';
    $progress = unserialize($raw);
    return BenchmarkResponse::json(['progress' => $progress]);
}
