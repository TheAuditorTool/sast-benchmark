require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_before_action_permit_bang
def handler(req)
  # Simulates permit! in before_action: admin key injected and passed through
  attrs = req.post_data.merge('admin' => true)
  user = User.create(attrs) # vuln-code-snippet vuln-line ruby_massassign_before_action_permit_bang
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_before_action_permit_bang
