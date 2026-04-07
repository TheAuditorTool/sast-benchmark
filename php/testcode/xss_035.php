<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_dom_textnode
function xss035(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('name');
    $dom = new DOMDocument();
    $p = $dom->createElement('p');
    $textNode = $dom->createTextNode($input); // vuln-code-snippet safe-line php_xss_dom_textnode
    $p->appendChild($textNode);
    $dom->appendChild($p);
    return BenchmarkResponse::html($dom->saveHTML());
}
// vuln-code-snippet end php_xss_dom_textnode
