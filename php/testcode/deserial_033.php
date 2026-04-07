<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_amf_object_inject
function amfDecode(string $binary): object {
    $len = unpack('N', substr($binary, 0, 4))[1];
    $className = substr($binary, 4, $len);
    $obj = new $className();
    $propData = substr($binary, 4 + $len);
    foreach (unpack('N*', $propData) as $k => $v) {
        $obj->{"prop$k"} = $v;
    }
    return $obj;
}

function deserial033(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $obj = amfDecode($body); // vuln-code-snippet vuln-line php_deser_amf_object_inject
    return BenchmarkResponse::ok('decoded');
}
// vuln-code-snippet end php_deser_amf_object_inject
