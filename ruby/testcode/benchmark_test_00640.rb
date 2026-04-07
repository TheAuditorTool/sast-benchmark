require_relative 'shared'

module Rails; module Application
  def self.credentials; OpenStruct.new(stripe_key: 'encrypted_value'); end
end; end

def create_charge_safe(req)
  amount = req.param('amount').to_i
  api_key = Rails::Application.credentials.stripe_key
  BenchmarkResponse.ok("charge: #{amount}")
end
