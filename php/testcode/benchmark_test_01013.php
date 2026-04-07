<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01013(BenchmarkRequest $req): BenchmarkResponse {
    $mailer = new stdClass();
    $mailer->Host = 'smtp.example.com';
    $mailer->Username = 'noreply@example.com';
    $mailer->Password = 'smtp_plain_password';
    $mailer->Port = 587;
    $to = $req->param('to');
    return BenchmarkResponse::ok('mail queued to ' . $to);
}
