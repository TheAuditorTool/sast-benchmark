<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_xxe_combo
function ssrf029(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml); // vuln-code-snippet vuln-line php_ssrf_xxe_combo
    $value = $dom->documentElement->textContent;
    return BenchmarkResponse::ok($value);
}
// vuln-code-snippet end php_ssrf_xxe_combo
