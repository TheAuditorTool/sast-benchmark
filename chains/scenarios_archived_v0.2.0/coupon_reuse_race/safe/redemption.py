"""Coupon redemption logic -- SAFE variant.

Uses a single atomic UPDATE WHERE used = 0 and checks affected row count.
If rowcount is 0 the coupon was already redeemed (or never existed after the
first check); no window exists for a concurrent request to also succeed.

Chain: concurrent POST /redeem -> atomic UPDATE WHERE used=0 -> rowcount check
Individual findings: none -- check-and-mark are atomic
Chain finding: none -- concurrent redemption is serialized by the database (CWE-362 mitigated)
"""
from coupons import get_db, get_coupon


def redeem_coupon(code, user_id):
    """
    Redeem a single-use coupon for user_id.

    SAFE: the UPDATE is conditional on used=0 in its own WHERE clause.
    If two requests race, only one will observe rowcount=1.
    """
    coupon = get_coupon(code)
    if not coupon:
        return {"error": "Coupon not found"}, 404

    conn = get_db()

# vuln-code-snippet start chain_coupon_reuse_safe
    cursor = conn.execute(
        "UPDATE coupons SET used = 1 WHERE code = ? AND used = 0", (code,)
    )  # vuln-code-snippet safe-line chain_coupon_reuse_safe
# vuln-code-snippet end chain_coupon_reuse_safe

    if cursor.rowcount == 0:
        conn.close()
        return {"error": "Coupon already redeemed"}, 409

    conn.execute(
        "INSERT INTO redemptions (coupon_code, user_id) VALUES (?, ?)",
        (code, user_id),
    )
    conn.commit()
    conn.close()

    return {"status": "ok", "discount_pct": coupon["discount_pct"]}, 200
