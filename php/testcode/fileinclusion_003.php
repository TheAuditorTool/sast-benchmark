<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_require_post
function fileinclusion003(BenchmarkRequest $req): BenchmarkResponse {
    $lang = $req->post('lang');
    require("lang/" . $lang . "/messages.php"); // vuln-code-snippet vuln-line php_fi_require_post
    return BenchmarkResponse::ok("language loaded");
}
// vuln-code-snippet end php_fi_require_post
