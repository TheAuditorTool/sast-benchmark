<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00805(BenchmarkRequest $req): BenchmarkResponse {
    $tmp = $_FILES['xml']['tmp_name'];
    $xml = file_get_contents($tmp);
    $dom = new DOMDocument();
    $dom->loadXML($xml);
    return BenchmarkResponse::ok($dom->saveXML());
}
