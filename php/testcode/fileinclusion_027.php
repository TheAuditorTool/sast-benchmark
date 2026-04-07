<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_cookie_lang_include
function fileinclusion027(BenchmarkRequest $req): BenchmarkResponse {
    $lang = $req->cookie('lang');
    include $lang . '/strings.php'; // vuln-code-snippet vuln-line php_fi_cookie_lang_include
    return BenchmarkResponse::ok('Strings loaded');
}
// vuln-code-snippet end php_fi_cookie_lang_include
