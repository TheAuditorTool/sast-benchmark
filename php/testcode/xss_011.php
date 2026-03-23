<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_heredoc
function xss_heredoc(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('greeting');

    $html = <<<HTML
    <html><body><p>{$input}</p></body></html>
    HTML; // vuln-code-snippet vuln-line php_xss_heredoc

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_heredoc
