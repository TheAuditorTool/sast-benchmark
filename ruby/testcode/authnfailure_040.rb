require_relative 'shared'
require 'rotp'

# vuln-code-snippet start ruby_authn_mfa_totp
def verify_totp(req, db)
  user_id = req.post('user_id')
  otp = req.post('otp')
  row = db.execute('SELECT totp_secret FROM users WHERE id = ?', [user_id]).first
  return BenchmarkResponse.error('User not found', 404) unless row
  secret = row[0]
  totp = ROTP::TOTP.new(secret)
  return BenchmarkResponse.error('Invalid OTP', 401) unless totp.verify(otp) # vuln-code-snippet safe-line ruby_authn_mfa_totp
  BenchmarkResponse.ok('MFA verified')
end
# vuln-code-snippet end ruby_authn_mfa_totp
