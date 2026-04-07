<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_splstack_typed
function deserial048(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('values'));
    $stack = new SplStack();
    foreach ($items as $item) {
        $val = filter_var($item, FILTER_VALIDATE_INT);
        if ($val === false) { // vuln-code-snippet safe-line php_deser_splstack_typed
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
// vuln-code-snippet end php_deser_splstack_typed
