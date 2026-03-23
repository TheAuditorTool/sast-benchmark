<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_htmlentities
function xss_htmlentities(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('comment');

    $safe = htmlentities($input, ENT_QUOTES, 'UTF-8'); // vuln-code-snippet safe-line php_xss_htmlentities
    $html = '<div class="comment">' . $safe . '</div>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_htmlentities
