require_relative 'shared'

module Vault
  module Logical
    def self.read(path); OpenStruct.new(data: { value: 'secret' }); end
  end
end

# vuln-code-snippet start ruby_hardcoded_vault
def get_secret(req)
  secret = Vault::Logical.read('secret/data/api_key') # vuln-code-snippet safe-line ruby_hardcoded_vault
  BenchmarkResponse.ok("key: #{secret.data[:value][0..3]}...")
end
# vuln-code-snippet end ruby_hardcoded_vault
