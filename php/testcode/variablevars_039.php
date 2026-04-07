<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_explicit_getter_setter
function variablevars039(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new stdClass();
    $obj->color = $req->param('color'); // vuln-code-snippet safe-line php_vv_explicit_getter_setter
    $obj->font = $req->param('font');
    return BenchmarkResponse::json(['color' => $obj->color, 'font' => $obj->font]);
}
// vuln-code-snippet end php_vv_explicit_getter_setter
