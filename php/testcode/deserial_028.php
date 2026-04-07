<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_upload_progress
function deserial028(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $key = $req->param('key');
    $prefix = ini_get('session.upload_progress.prefix');
    $progressKey = $prefix . $key;
    $raw = $_SESSION[$progressKey] ?? '';
    $progress = unserialize($raw); // vuln-code-snippet vuln-line php_deser_upload_progress
    return BenchmarkResponse::json(['progress' => $progress]);
}
// vuln-code-snippet end php_deser_upload_progress
