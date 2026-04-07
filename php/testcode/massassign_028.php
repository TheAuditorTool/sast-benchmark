<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_graphql_spread
function massassign028(BenchmarkRequest $req): BenchmarkResponse {
    $args = json_decode($req->bodyStr(), true)['args'];
    $model = new stdClass();
    foreach ($args as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_graphql_spread
    }
    return BenchmarkResponse::ok('resolved');
}
// vuln-code-snippet end php_massassign_graphql_spread
