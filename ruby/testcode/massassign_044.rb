require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

ACCESSIBLE = %w[name email].freeze

# vuln-code-snippet start ruby_massassign_attr_accessible_rails3
def handler(req)
  user = User.create(req.post_data.slice(*ACCESSIBLE)) # vuln-code-snippet safe-line ruby_massassign_attr_accessible_rails3
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_attr_accessible_rails3
