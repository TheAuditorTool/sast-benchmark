require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_nested_without_permit
def handler(req)
  # Nested attributes passed through without filtering the nested hash
  user = User.new(profile_attributes: req.post_data) # vuln-code-snippet vuln-line ruby_massassign_nested_without_permit
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_nested_without_permit
