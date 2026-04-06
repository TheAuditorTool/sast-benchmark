<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xslt_trusted_file
function xxe016(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    $xml = new DOMDocument();
    $root = $xml->createElement('data');
    foreach ($data as $key => $val) {
        $child = $xml->createElement($key, htmlspecialchars($val, ENT_XML1));
        $root->appendChild($child);
    }
    $xml->appendChild($root);
    $xsl = new DOMDocument();
    $xsl->load(__DIR__ . '/templates/trusted_transform.xsl'); // vuln-code-snippet safe-line php_xxe_xslt_trusted_file
    $proc = new XSLTProcessor();
    $proc->importStylesheet($xsl);
    $output = $proc->transformToXml($xml);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_xxe_xslt_trusted_file
