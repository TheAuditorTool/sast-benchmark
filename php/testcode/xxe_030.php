<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_zip_xml_extract
function xxe030(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = '/var/app/uploads/' . basename($req->param('zip'));
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $xml = $zip->getFromName('content.xml');
    $dom = new DOMDocument();
    $dom->loadXML($xml); // vuln-code-snippet vuln-line php_xxe_zip_xml_extract
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_zip_xml_extract
