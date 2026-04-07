<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00043(BenchmarkRequest $req): BenchmarkResponse {
    $xslPath = $req->param('xsl');
    $xsl = new DOMDocument();
    $xsl->load($xslPath);
    $proc = new XSLTProcessor();
    $proc->importStylesheet($xsl);
    return BenchmarkResponse::ok('imported');
}
