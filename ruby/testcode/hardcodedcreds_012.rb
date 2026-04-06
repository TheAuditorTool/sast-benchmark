require_relative 'shared'

module Rails; module Application
  def self.credentials; OpenStruct.new(stripe_key: 'encrypted_value'); end
end; end

# vuln-code-snippet start ruby_hardcoded_rails_creds
def create_charge_safe(req)
  amount = req.param('amount').to_i
  api_key = Rails::Application.credentials.stripe_key # vuln-code-snippet safe-line ruby_hardcoded_rails_creds
  BenchmarkResponse.ok("charge: #{amount}")
end
# vuln-code-snippet end ruby_hardcoded_rails_creds
