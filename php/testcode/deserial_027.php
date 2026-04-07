<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_webhook_no_sig
function deserial027(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $event = unserialize($body); // vuln-code-snippet vuln-line php_deser_webhook_no_sig
    processWebhookEvent($event);
    return BenchmarkResponse::ok('received');
}
// vuln-code-snippet end php_deser_webhook_no_sig
