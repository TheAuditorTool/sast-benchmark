<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00235(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $sig = $req->header('X-XML-Signature');
    if (!openssl_verify($xml, base64_decode($sig), file_get_contents('/etc/app/xml.pub'))) {
        return BenchmarkResponse::badRequest('invalid sig');
    }
    libxml_disable_entity_loader(true);
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
