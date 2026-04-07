<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_slack_webhook_token
function hardcodedcreds029(BenchmarkRequest $req): BenchmarkResponse {
    $webhookUrl = 'https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX'; // vuln-code-snippet vuln-line php_hardcoded_slack_webhook_token
    $message = $req->param('message');
    $payload = json_encode(['text' => $message]);
    $ch = curl_init($webhookUrl);
    curl_setopt($ch, CURLOPT_POSTFIELDS, $payload);
    curl_exec($ch);
    return BenchmarkResponse::ok('sent');
}
// vuln-code-snippet end php_hardcoded_slack_webhook_token
