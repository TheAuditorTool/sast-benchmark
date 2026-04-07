require_relative 'shared'

module Vault
  module Logical
    def self.read(path); OpenStruct.new(data: { value: 'secret' }); end
  end
end

def get_secret(req)
  secret = Vault::Logical.read('secret/data/api_key')
  BenchmarkResponse.ok("key: #{secret.data[:value][0..3]}...")
end
