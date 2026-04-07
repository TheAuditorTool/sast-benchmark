<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_scientific_notation_pair
function typejuggling030(BenchmarkRequest $req): BenchmarkResponse {
    $a = $req->param('a');
    $b = $req->param('b');
    if ($a == $b) { // vuln-code-snippet vuln-line php_tj_scientific_notation_pair // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('same');
    }
    return BenchmarkResponse::ok('different');
}
// vuln-code-snippet end php_tj_scientific_notation_pair
