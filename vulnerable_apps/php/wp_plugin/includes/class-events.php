<?php
// WordPress Events Manager Plugin - Event CRUD operations
require_once __DIR__ . '/../../../testcode/shared.php';

$wpdb = new FakeWpdb(getDbConnection());

// vuln-code-snippet start wp_sqli_update
function wp_sqli_update(FakeWpdb $wpdb, BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $id = $req->post('id');
    $wpdb->query("UPDATE wp_events SET name='" . $name . "' WHERE id=" . $id); // vuln-code-snippet vuln-line wp_sqli_update
    return BenchmarkResponse::ok('Updated');
}
// vuln-code-snippet end wp_sqli_update

// vuln-code-snippet start wp_sqli_update_safe
function wp_sqli_update_safe(FakeWpdb $wpdb, BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $id = $req->post('id');
    $sql = $wpdb->prepare("UPDATE wp_events SET name=%s WHERE id=%s", $name, $id); // vuln-code-snippet safe-line wp_sqli_update_safe
    $wpdb->query($sql);
    return BenchmarkResponse::ok('Updated');
}
// vuln-code-snippet end wp_sqli_update_safe

// vuln-code-snippet start wp_xss_event_list
function wp_xss_event_list(FakeWpdb $wpdb): BenchmarkResponse {
    $events = $wpdb->get_results("SELECT * FROM wp_events LIMIT 50");
    $html = '<table>';
    foreach ($events as $event) {
        $html .= '<td>' . $event->description . '</td>'; // vuln-code-snippet vuln-line wp_xss_event_list
    }
    $html .= '</table>';
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end wp_xss_event_list

// vuln-code-snippet start wp_xss_event_escaped
function wp_xss_event_escaped(FakeWpdb $wpdb): BenchmarkResponse {
    $events = $wpdb->get_results("SELECT * FROM wp_events LIMIT 50");
    $html = '<table>';
    foreach ($events as $event) {
        $html .= '<td>' . esc_html($event->description) . '</td>'; // vuln-code-snippet safe-line wp_xss_event_escaped
    }
    $html .= '</table>';
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end wp_xss_event_escaped

// vuln-code-snippet start wp_extract_atts
function wp_extract_atts(BenchmarkRequest $req): BenchmarkResponse {
    $user_atts = $req->queryParams;
    extract($user_atts); // vuln-code-snippet vuln-line wp_extract_atts
    return BenchmarkResponse::ok(isset($title) ? $title : 'Events');
}
// vuln-code-snippet end wp_extract_atts

// vuln-code-snippet start wp_extract_safe
function wp_extract_safe(BenchmarkRequest $req): BenchmarkResponse {
    $user_atts = $req->queryParams;
    $defaults = ['title' => 'Events', 'limit' => 10];
    $atts = array_merge($defaults, array_intersect_key($user_atts, $defaults)); // vuln-code-snippet safe-line wp_extract_safe
    return BenchmarkResponse::ok($atts['title']);
}
// vuln-code-snippet end wp_extract_safe
