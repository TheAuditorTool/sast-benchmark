<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00942(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    libxml_disable_entity_loader(true);
    $doc = simplexml_load_string($xml, 'SimpleXMLElement', LIBXML_NONET);
    if ($doc === false) {
        return BenchmarkResponse::badRequest('invalid xml');
    }
    return BenchmarkResponse::ok((string) $doc->name);
}
