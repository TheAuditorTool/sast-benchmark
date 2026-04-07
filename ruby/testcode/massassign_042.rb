require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

PERMITTED = %i[name email location].freeze

# vuln-code-snippet start ruby_massassign_action_serializer_input
def handler(req)
  attrs = req.post_data.slice(*PERMITTED.map(&:to_s))
  profile = User.create(attrs) # vuln-code-snippet safe-line ruby_massassign_action_serializer_input
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_action_serializer_input
