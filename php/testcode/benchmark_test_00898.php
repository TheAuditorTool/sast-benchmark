<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00898(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    $xml = new DOMDocument();
    $root = $xml->createElement('data');
    foreach ($data as $key => $val) {
        $child = $xml->createElement($key, htmlspecialchars($val, ENT_XML1));
        $root->appendChild($child);
    }
    $xml->appendChild($root);
    $xsl = new DOMDocument();
    $xsl->load(__DIR__ . '/templates/trusted_transform.xsl');
    $proc = new XSLTProcessor();
    $proc->importStylesheet($xsl);
    $output = $proc->transformToXml($xml);
    return BenchmarkResponse::ok($output);
}
