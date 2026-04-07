<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_server_written_session
function deserial043(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $userId = (int) ($_SESSION['user_id'] ?? 0);
    $prefs = $_SESSION['preferences'] ?? []; // vuln-code-snippet safe-line php_deser_server_written_session
    return BenchmarkResponse::json(['user' => $userId, 'prefs' => $prefs]);
}
// vuln-code-snippet end php_deser_server_written_session
