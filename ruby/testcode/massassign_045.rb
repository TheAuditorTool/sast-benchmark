require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

REQUIRED = %w[email password].freeze

# vuln-code-snippet start ruby_massassign_contract_grape
def handler(req)
  raise 'missing required params' unless REQUIRED.all? { |k| req.post_data.key?(k) }
  user = User.create(req.post_data.slice(*REQUIRED)) # vuln-code-snippet safe-line ruby_massassign_contract_grape
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_contract_grape
