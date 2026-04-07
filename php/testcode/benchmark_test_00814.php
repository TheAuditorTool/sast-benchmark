<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00814(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = new PDO('sqlite:/tmp/webhooks.db');
    $stmt = $db->prepare('SELECT url FROM webhooks WHERE id = ?');
    $stmt->execute([$id]);
    $webhookUrl = $stmt->fetchColumn();
    $content = file_get_contents($webhookUrl);
    return BenchmarkResponse::ok($content);
}
