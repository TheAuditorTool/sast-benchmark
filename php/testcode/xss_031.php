<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_ent_xml1
function xss031(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('val');
    $html = "<data>" . htmlspecialchars($input, ENT_XML1, 'UTF-8') . "</data>"; // vuln-code-snippet safe-line php_xss_ent_xml1
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_ent_xml1
