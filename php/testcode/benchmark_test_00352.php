<?php
require_once __DIR__ . '/shared.php';

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

function benchmarkTest00352(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $obj = amfDecode($body);
    return BenchmarkResponse::ok('decoded');
}
