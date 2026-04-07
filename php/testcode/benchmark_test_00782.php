<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00782(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new stdClass();
    $obj->color = $req->param('color');
    $obj->font = $req->param('font');
    return BenchmarkResponse::json(['color' => $obj->color, 'font' => $obj->font]);
}
