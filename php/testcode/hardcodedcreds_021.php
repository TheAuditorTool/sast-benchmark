<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_smtp_password
function hardcodedcreds021(BenchmarkRequest $req): BenchmarkResponse {
    $mailer = new stdClass();
    $mailer->Host = 'smtp.example.com';
    $mailer->Username = 'noreply@example.com';
    $mailer->Password = 'smtp_plain_password'; // vuln-code-snippet vuln-line php_hardcoded_smtp_password
    $mailer->Port = 587;
    $to = $req->param('to');
    return BenchmarkResponse::ok('mail queued to ' . $to);
}
// vuln-code-snippet end php_hardcoded_smtp_password
