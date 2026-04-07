<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00841(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = '/var/app/uploads/' . basename($req->param('zip'));
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $xml = $zip->getFromName('content.xml');
    $dom = new DOMDocument();
    $dom->loadXML($xml);
    return BenchmarkResponse::ok($dom->saveXML());
}
