require_relative 'shared'

# vuln-code-snippet start ruby_csrf_otp_reauth
def withdraw_funds_otp(req)
  otp_code = req.post('otp')
  user_id = req.param('user_id').to_i
  db = get_db_connection
  secret = db.execute("SELECT totp_secret FROM users WHERE id = ?", [user_id]).first&.dig(0)
  totp = ROTP::TOTP.new(secret.to_s)
  return BenchmarkResponse.bad_request('invalid OTP') unless totp.verify(otp_code.to_s)  # vuln-code-snippet safe-line ruby_csrf_otp_reauth
  amount = req.post('amount').to_f
  BenchmarkResponse.json({ result: amount })
end
# vuln-code-snippet end ruby_csrf_otp_reauth
