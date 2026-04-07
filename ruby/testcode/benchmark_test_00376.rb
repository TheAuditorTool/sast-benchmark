require_relative 'shared'

class UserForm
  include Virtus.model
  attribute :name, String
  attribute :age,  Integer
end

def virtus_coerce_handler(req)
  data = JSON.parse(req.post('data'))
  form = UserForm.new(data)
  BenchmarkResponse.json({ result: form.name })
end
