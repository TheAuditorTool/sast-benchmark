<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00209(BenchmarkRequest $req): BenchmarkResponse {
    $nonce = $req->post('_wpnonce');
    $action = 'update_profile';
    if (!wp_verify_nonce($nonce, $action)) {
        return BenchmarkResponse::badRequest('nonce verification failed');
    }
    updateProfile($req->post('data'));
    $newNonce = wp_create_nonce($action);
    return BenchmarkResponse::json(['ok' => true, 'nonce' => $newNonce]);
}
