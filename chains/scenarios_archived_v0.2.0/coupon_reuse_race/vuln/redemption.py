"""Coupon redemption logic -- VULNERABLE variant.

Reads the used flag with a SELECT, then in a separate UPDATE marks the coupon used.
Two concurrent requests for the same coupon both read used=0, both pass the check,
and both commit their UPDATE -- each receiving a valid discount.

Chain: concurrent POST /redeem -> SELECT used=0 (passes) -> [race window] -> UPDATE used=1
Individual findings: non-atomic check-and-mark (medium)
Chain finding: single-use coupon redeemed multiple times (CWE-362, critical)
"""
from coupons import get_db, get_coupon


def redeem_coupon(code, user_id):
    """
    Redeem a single-use coupon for user_id.

    VULNERABLE: the check (used == 0) and the update (used = 1) are separate queries.
    A concurrent redemption can observe the same used=0 state before either commits.
    """
    coupon = get_coupon(code)
    if not coupon:
        return {"error": "Coupon not found"}, 404

    # TOCTOU: another request can pass this check before either marks the coupon used
    if coupon["used"] != 0:
        return {"error": "Coupon already redeemed"}, 409

    conn = get_db()

# vuln-code-snippet start chain_coupon_reuse_vuln
    conn.execute(
        "UPDATE coupons SET used = 1 WHERE code = ?", (code,)
    )  # vuln-code-snippet vuln-line chain_coupon_reuse_vuln
# vuln-code-snippet end chain_coupon_reuse_vuln

    conn.execute(
        "INSERT INTO redemptions (coupon_code, user_id) VALUES (?, ?)",
        (code, user_id),
    )
    conn.commit()
    conn.close()

    return {"status": "ok", "discount_pct": coupon["discount_pct"]}, 200
