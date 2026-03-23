<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_require_basename
function fileinclusion004(BenchmarkRequest $req): BenchmarkResponse {
    $lang = $req->post('lang');
    $safeLang = basename($lang);
    require("lang/" . $safeLang . "/messages.php"); // vuln-code-snippet safe-line php_fi_require_basename
    return BenchmarkResponse::ok("language loaded");
}
// vuln-code-snippet end php_fi_require_basename
