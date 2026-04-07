<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_webhook_stored_fetch
function ssrf028(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = new PDO('sqlite:/tmp/webhooks.db');
    $stmt = $db->prepare('SELECT url FROM webhooks WHERE id = ?');
    $stmt->execute([$id]);
    $webhookUrl = $stmt->fetchColumn();
    $content = file_get_contents($webhookUrl); // vuln-code-snippet vuln-line php_ssrf_webhook_stored_fetch
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_webhook_stored_fetch
