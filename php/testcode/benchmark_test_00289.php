<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00289(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('values'));
    $stack = new SplStack();
    foreach ($items as $item) {
        $val = filter_var($item, FILTER_VALIDATE_INT);
        if ($val === false) {
            return BenchmarkResponse::badRequest('non-integer value rejected');
        }
        $stack->push($val);
    }
    $result = [];
    foreach ($stack as $v) {
        $result[] = $v;
    }
    return BenchmarkResponse::json($result);
}
