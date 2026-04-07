<?php
// vuln_blog - Comment handling
require_once __DIR__ . '/config.php';

// vuln-code-snippet start vb_xss_comment
function renderCommentUnsafe(array $comment): void {
    echo '<div class="comment">';
    echo '<p>' . $comment['body'] . '</p>'; // vuln-code-snippet vuln-line vb_xss_comment
    echo '<small>by ' . $comment['author'] . '</small>';
    echo '</div>';
}
// vuln-code-snippet end vb_xss_comment

// vuln-code-snippet start vb_xss_comment_safe
function renderCommentSafe(array $comment): void {
    echo '<div class="comment">';
    echo '<p>' . htmlspecialchars($comment['body'], ENT_QUOTES, 'UTF-8') . '</p>'; // vuln-code-snippet safe-line vb_xss_comment_safe
    echo '<small>by ' . htmlspecialchars($comment['author'], ENT_QUOTES, 'UTF-8') . '</small>';
    echo '</div>';
}
// vuln-code-snippet end vb_xss_comment_safe

// vuln-code-snippet start vb_sqli_insert_comment
function insertCommentUnsafe(string $postId, string $author, string $body): void {
    $pdo = getDbConnectionHardcoded();
    $query = "INSERT INTO comments (post_id, author, body) VALUES ('" . $postId . "', '" . $author . "', '" . $body . "')"; // vuln-code-snippet vuln-line vb_sqli_insert_comment
    $pdo->exec($query);
}
// vuln-code-snippet end vb_sqli_insert_comment

// vuln-code-snippet start vb_sqli_insert_safe
function insertCommentSafe(string $postId, string $author, string $body): void {
    $pdo = getDbConnectionEnv();
    $stmt = $pdo->prepare("INSERT INTO comments (post_id, author, body) VALUES (?, ?, ?)"); // vuln-code-snippet safe-line vb_sqli_insert_safe
    $stmt->execute([$postId, $author, $body]);
}
// vuln-code-snippet end vb_sqli_insert_safe

// vuln-code-snippet start vb_csrf_no_token
function handleCommentPostNoCsrf(): void {
    $postId = $_POST['post_id'];
    $author = $_POST['author'];
    $body = $_POST['body'];
    insertCommentSafe($postId, $author, $body); // vuln-code-snippet vuln-line vb_csrf_no_token
    header('Location: /post/' . $postId);
}
// vuln-code-snippet end vb_csrf_no_token

// vuln-code-snippet start vb_csrf_verified
function handleCommentPostWithCsrf(): void {
    $token = $_POST['csrf_token'] ?? '';
    $expected = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($expected, $token)) { // vuln-code-snippet safe-line vb_csrf_verified
        http_response_code(403);
        echo 'CSRF token mismatch';
        return;
    }
    $postId = $_POST['post_id'];
    $author = $_POST['author'];
    $body = $_POST['body'];
    insertCommentSafe($postId, $author, $body);
    header('Location: /post/' . $postId);
}
// vuln-code-snippet end vb_csrf_verified
