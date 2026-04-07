<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00082(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('name');
    $dom = new DOMDocument();
    $p = $dom->createElement('p');
    $textNode = $dom->createTextNode($input);
    $p->appendChild($textNode);
    $dom->appendChild($p);
    return BenchmarkResponse::html($dom->saveHTML());
}
