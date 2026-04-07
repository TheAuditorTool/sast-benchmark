<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_mail_fifth_arg
function cmdi021(BenchmarkRequest $req): BenchmarkResponse {
    $to = $req->param('to');
    $subject = $req->param('subject');
    $body = $req->param('body');
    $from = $req->param('from');
    mail($to, $subject, $body, '', "-f $from"); // vuln-code-snippet vuln-line php_cmdi_mail_fifth_arg
    return BenchmarkResponse::ok('sent');
}
// vuln-code-snippet end php_cmdi_mail_fifth_arg
