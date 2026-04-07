<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00014(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $xsl = file_get_contents(__DIR__ . '/templates/transform.xsl');
    $xmlDoc = new DOMDocument();
    $xmlDoc->loadXML($xml);
    $xslDoc = new DOMDocument();
    $xslDoc->loadXML($xsl);
    $proc = new XSLTProcessor();
    $proc->importStylesheet($xslDoc);
    $output = $proc->transformToXml($xmlDoc);
    return BenchmarkResponse::ok($output);
}
