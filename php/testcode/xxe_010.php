<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xslt_user_xml
function xxe010(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $xsl = file_get_contents(__DIR__ . '/templates/transform.xsl');
    $xmlDoc = new DOMDocument();
    $xmlDoc->loadXML($xml); // vuln-code-snippet vuln-line php_xxe_xslt_user_xml
    $xslDoc = new DOMDocument();
    $xslDoc->loadXML($xsl);
    $proc = new XSLTProcessor();
    $proc->importStylesheet($xslDoc);
    $output = $proc->transformToXml($xmlDoc);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_xxe_xslt_user_xml
