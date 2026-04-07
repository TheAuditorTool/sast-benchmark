<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_preg_callback_eval
function codeinj031(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->param('code');
    $str = 'hello world';
    $result = preg_replace_callback('/(.+)/', function ($m) use ($code) {
        return eval($code); // vuln-code-snippet vuln-line php_codeinj_preg_callback_eval
    }, $str);
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_preg_callback_eval
