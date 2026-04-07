require_relative 'shared'

User = Struct.new(:name, :email, :role, :bio, :admin, keyword_init: true)
class << User; def create(h); new(**h.transform_keys(&:to_sym).slice(:name,:email,:role,:bio,:admin)); end; end

# vuln-code-snippet start ruby_massassign_form_obj_unfiltered
def handler(req)
  # Form object initialized with the full unfiltered params hash
  form = OpenStruct.new(req.post_data)
  user = User.create(form.to_h) # vuln-code-snippet vuln-line ruby_massassign_form_obj_unfiltered
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_massassign_form_obj_unfiltered
