require_relative 'shared'

class UserForm
  include Virtus.model
  attribute :name, String
  attribute :age,  Integer
end

# vuln-code-snippet start ruby_deser_virtus_coerce
def virtus_coerce_handler(req)
  data = JSON.parse(req.post('data'))  # vuln-code-snippet safe-line ruby_deser_virtus_coerce
  form = UserForm.new(data)
  BenchmarkResponse.json({ result: form.name })
end
# vuln-code-snippet end ruby_deser_virtus_coerce
