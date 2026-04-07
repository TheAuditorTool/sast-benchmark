<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_wp_nonce_rotated
function csrf037(BenchmarkRequest $req): BenchmarkResponse {
    $nonce = $req->post('_wpnonce');
    $action = 'update_profile';
    if (!wp_verify_nonce($nonce, $action)) { // vuln-code-snippet safe-line php_csrf_wp_nonce_rotated
        return BenchmarkResponse::badRequest('nonce verification failed');
    }
    updateProfile($req->post('data'));
    $newNonce = wp_create_nonce($action);
    return BenchmarkResponse::json(['ok' => true, 'nonce' => $newNonce]);
}
// vuln-code-snippet end php_csrf_wp_nonce_rotated
