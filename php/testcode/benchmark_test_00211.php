<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00211(BenchmarkRequest $req): BenchmarkResponse {
    $to = $req->param('to');
    $subject = $req->param('subject');
    $body = $req->param('body');
    $from = $req->param('from');
    mail($to, $subject, $body, '', "-f $from");
    return BenchmarkResponse::ok('sent');
}
